#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPOSE_ARGS=(
  -f "${ROOT_DIR}/tests/e2e/compose.direct-openvasd.yaml"
  -f "${ROOT_DIR}/tests/e2e/compose.enterprise-agent-control.yaml"
)

if [[ -z "${GVM_GATEWAY_E2E_RUN_SCOPE:-}" ]]; then
  echo "GVM_GATEWAY_E2E_RUN_SCOPE is required for isolated enterprise Compose state" >&2
  exit 2
fi
if [[ ! "${GVM_GATEWAY_E2E_RUN_SCOPE}" =~ ^[A-Za-z0-9][A-Za-z0-9_.-]*$ ]]; then
  echo "GVM_GATEWAY_E2E_RUN_SCOPE contains unsupported characters" >&2
  exit 2
fi
export COMPOSE_PROJECT_NAME="rust-gvm-api-enterprise-${GVM_GATEWAY_E2E_RUN_SCOPE}"

FEED_RESOLUTION_PATH="${GVM_GATEWAY_E2E_FEED_RESOLUTION_PATH:-${ROOT_DIR}/dist/e2e-enterprise/feed-resolution.json}"
FEED_ENV_PATH="${FEED_RESOLUTION_PATH}.compose.env"

load_feed_environment() {
  local path="$1"
  local count=0
  local name value
  local -A seen=()
  while IFS='=' read -r name value; do
    case "${name}" in
      GVM_GATEWAY_E2E_VULNERABILITY_TESTS_IMAGE|GVM_GATEWAY_E2E_DATA_OBJECTS_IMAGE|GVM_GATEWAY_E2E_REPORT_FORMATS_IMAGE|GVM_GATEWAY_E2E_GPG_DATA_IMAGE|GVM_GATEWAY_E2E_NOTUS_DATA_IMAGE)
        ;;
      *)
        echo "Invalid rolling feed environment name in ${path}: ${name}" >&2
        return 1
        ;;
    esac
    if [[ ! "${value}" =~ ^.+:[^/@]+@sha256:[0-9a-f]{64}$ ]]; then
      echo "Rolling feed environment ${name} is not an exact tag@digest reference" >&2
      return 1
    fi
    if [[ -n "${seen[${name}]+present}" ]]; then
      echo "Rolling feed environment declares ${name} more than once" >&2
      return 1
    fi
    seen["${name}"]=1
    export "${name}=${value}"
    count=$((count + 1))
  done < "${path}"
  [[ "${count}" -eq 5 ]] || {
    echo "Rolling feed environment must contain exactly five resolved images" >&2
    return 1
  }
}

set_cleanup_feed_placeholders() {
  local digest="sha256:$(printf '0%.0s' {1..64})"
  export GVM_GATEWAY_E2E_VULNERABILITY_TESTS_IMAGE="invalid.invalid/vulnerability-tests:cleanup@${digest}"
  export GVM_GATEWAY_E2E_DATA_OBJECTS_IMAGE="invalid.invalid/data-objects:cleanup@${digest}"
  export GVM_GATEWAY_E2E_REPORT_FORMATS_IMAGE="invalid.invalid/report-formats:cleanup@${digest}"
  export GVM_GATEWAY_E2E_GPG_DATA_IMAGE="invalid.invalid/gpg-data:cleanup@${digest}"
  export GVM_GATEWAY_E2E_NOTUS_DATA_IMAGE="invalid.invalid/notus-data:cleanup@${digest}"
}

COMPOSE_COMMAND=""
for compose_argument in "$@"; do
  case "${compose_argument}" in
    up|start|create|run|restart)
      COMPOSE_COMMAND="${compose_argument}"
      break
      ;;
  esac
done

if [[ -n "${COMPOSE_COMMAND}" ]]; then
  : "${GVM_GATEWAY_E2E_AGENT_CONTROL_IMAGE:?exact Agent Control image digest is required}"
  : "${GVM_GATEWAY_E2E_AGENT_CONTROL_SOURCE:?exact Agent Control source checkout is required}"
  : "${GVM_GATEWAY_E2E_AGENT_CONTROL_BUILD_MANIFEST:?exact Agent Control build manifest is required}"
  [[ "${GVM_GATEWAY_E2E_AGENT_CONTROL_IMAGE}" =~ @sha256:[0-9a-f]{64}$ ]] || {
    echo "Agent Control image must be an immutable repository digest" >&2
    exit 2
  }
  python3 "${ROOT_DIR}/scripts/validate-direct-openvasd-pins.py"
  python3 "${ROOT_DIR}/scripts/validate-enterprise-agent-control-inputs.py"
  if [[ "${COMPOSE_COMMAND}" == "up" || ! -f "${FEED_ENV_PATH}" ]]; then
    python3 "${ROOT_DIR}/scripts/direct_openvasd_feed_resolution.py" \
      --output "${FEED_RESOLUTION_PATH}" \
      --compose-env "${FEED_ENV_PATH}"
  fi
  load_feed_environment "${FEED_ENV_PATH}"
else
  if [[ -f "${FEED_ENV_PATH}" ]]; then
    load_feed_environment "${FEED_ENV_PATH}"
  else
    set_cleanup_feed_placeholders
  fi
fi

if docker compose version >/dev/null 2>&1; then
  exec docker compose "${COMPOSE_ARGS[@]}" --profile direct-openvasd --profile enterprise-agent-control "$@"
fi
if command -v podman-compose >/dev/null 2>&1; then
  exec podman-compose "${COMPOSE_ARGS[@]}" --profile direct-openvasd --profile enterprise-agent-control "$@"
fi
echo "Neither docker compose nor podman-compose is available" >&2
exit 1
