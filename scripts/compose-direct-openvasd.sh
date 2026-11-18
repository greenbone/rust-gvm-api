#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
COMPOSE_ARGS=(-f "${ROOT_DIR}/tests/e2e/compose.direct-openvasd.yaml")
OPENVASD_KIND="${GVM_GATEWAY_E2E_OPENVASD_KIND:-emulator}"
if [[ "${OPENVASD_KIND}" == "real" ]]; then
  COMPOSE_ARGS+=(-f "${ROOT_DIR}/tests/e2e/compose.real-openvasd.yaml")
fi

RUN_SCOPE="${GVM_GATEWAY_E2E_RUN_SCOPE:-local-${UID}}"
RUN_SCOPE="$(printf '%s' "${RUN_SCOPE}" | tr -c 'a-zA-Z0-9_-' '-')"
PROJECT_NAME="rust-gvm-api-openvasd-${RUN_SCOPE}"

if [[ $# -eq 0 ]]; then
  echo "Usage: scripts/compose-direct-openvasd.sh <compose-args...>" >&2
  exit 1
fi

if [[ -n "${GVM_GATEWAY_E2E_FEED_RESOLUTION_PATH:-}" ]]; then
  FEED_RESOLUTION_PATH="${GVM_GATEWAY_E2E_FEED_RESOLUTION_PATH}"
elif [[ "${OPENVASD_KIND}" == "real" ]]; then
  FEED_RESOLUTION_PATH="${ROOT_DIR}/dist/e2e-openvasd/real/feed-resolution.json"
else
  FEED_RESOLUTION_PATH="${ROOT_DIR}/dist/e2e-openvasd/raw/feed-resolution.json"
fi
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
  if [[ "${count}" -ne 5 ]]; then
    echo "Rolling feed environment must contain exactly five resolved images" >&2
    return 1
  fi
}

set_cleanup_feed_placeholders() {
  local digest="sha256:$(printf '0%.0s' {1..64})"
  export GVM_GATEWAY_E2E_VULNERABILITY_TESTS_IMAGE="invalid.invalid/vulnerability-tests:cleanup@${digest}"
  export GVM_GATEWAY_E2E_DATA_OBJECTS_IMAGE="invalid.invalid/data-objects:cleanup@${digest}"
  export GVM_GATEWAY_E2E_REPORT_FORMATS_IMAGE="invalid.invalid/report-formats:cleanup@${digest}"
  export GVM_GATEWAY_E2E_GPG_DATA_IMAGE="invalid.invalid/gpg-data:cleanup@${digest}"
  export GVM_GATEWAY_E2E_NOTUS_DATA_IMAGE="invalid.invalid/notus-data:cleanup@${digest}"
}

# Validate immutable executable inputs before every command that can start
# containers. Resolve rolling feed channels at the initial `up`, or when no
# successful resolution exists yet, then reuse that exact resolution for later
# `run`/`start`/`restart` invocations in the same E2E run. Scan all arguments so
# a Compose global option before the command cannot bypass the policy. Read and
# cleanup commands use the same resolution or inert exact placeholders.
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
    VALIDATOR_ARGS=(
      --openvasd-kind
      "${OPENVASD_KIND}"
    )
    if [[ -n "${GVM_GATEWAY_E2E_REAL_OPENVASD_IMAGE:-}" || \
          -n "${GVM_GATEWAY_E2E_TEMPORARY_SCANNER_BUILD_MANIFEST:-}" ]]; then
      VALIDATOR_ARGS+=(
        --temporary-scanner-build-manifest
        "${GVM_GATEWAY_E2E_TEMPORARY_SCANNER_BUILD_MANIFEST}"
        --real-openvasd-image
        "${GVM_GATEWAY_E2E_REAL_OPENVASD_IMAGE}"
      )
    fi
    python3 "${ROOT_DIR}/scripts/validate-direct-openvasd-pins.py" "${VALIDATOR_ARGS[@]}"
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

if command -v docker >/dev/null 2>&1 && docker compose version >/dev/null 2>&1; then
  exec docker compose --project-name "${PROJECT_NAME}" "${COMPOSE_ARGS[@]}" \
    --profile direct-openvasd "$@"
fi

echo "The direct-OpenVASD lane requires Docker Compose." >&2
exit 1
