#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: binary-archive-smoke.sh --archive <path> --version <semver> --architecture <amd64|arm64>
EOF
}

ARCHIVE=""
VERSION=""
ARCHITECTURE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --archive)
      ARCHIVE="${2:-}"
      shift 2
      ;;
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    --architecture)
      ARCHITECTURE="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ -z "${ARCHIVE}" || -z "${VERSION}" || -z "${ARCHITECTURE}" ]]; then
  usage >&2
  exit 1
fi

ARCHIVE="$(realpath "${ARCHIVE}")"
ARCHIVE_DIR="$(dirname "${ARCHIVE}")"
ARCHIVE_NAME="gvm-gateway_${VERSION}_linux_${ARCHITECTURE}.tar.gz"
ARCHIVE_ROOT_NAME="gvm-gateway-${VERSION}-linux-${ARCHITECTURE}"

if [[ "$(basename "${ARCHIVE}")" != "${ARCHIVE_NAME}" ]]; then
  echo "unexpected archive name: $(basename "${ARCHIVE}")" >&2
  exit 1
fi

if [[ ! -f "${ARCHIVE}.sha256" ]]; then
  echo "archive checksum missing: ${ARCHIVE}.sha256" >&2
  exit 1
fi

# This consumer-shaped check proves that the published checksum is portable:
# it must reference the asset basename rather than a runner-specific path.
CHECKSUM_TARGET="$(awk 'NR == 1 { print $2 }' "${ARCHIVE}.sha256")"
if [[ "${CHECKSUM_TARGET}" != "${ARCHIVE_NAME}" ]]; then
  echo "checksum must reference archive basename, got: ${CHECKSUM_TARGET}" >&2
  exit 1
fi
(
  cd "${ARCHIVE_DIR}"
  sha256sum --check --strict "${ARCHIVE_NAME}.sha256"
)

while IFS= read -r entry; do
  case "${entry}" in
    "${ARCHIVE_ROOT_NAME}"|"${ARCHIVE_ROOT_NAME}/"*)
      ;;
    *)
      echo "archive contains entry outside expected root: ${entry}" >&2
      exit 1
      ;;
  esac
done < <(tar -tzf "${ARCHIVE}")

TEMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TEMP_DIR}"' EXIT
tar -xzf "${ARCHIVE}" -C "${TEMP_DIR}"
ARCHIVE_ROOT="${TEMP_DIR}/${ARCHIVE_ROOT_NAME}"

# These assertions define the standalone distribution contract used by GOS:
# executable, example configuration, operator docs, license, and provenance.
test -x "${ARCHIVE_ROOT}/gvm-gateway"
test -f "${ARCHIVE_ROOT}/gvm-gateway.toml.example"
test -f "${ARCHIVE_ROOT}/README.md"
test -f "${ARCHIVE_ROOT}/LICENSE"
test -f "${ARCHIVE_ROOT}/BUILDINFO"
grep -Fx "artifact_name=${ARCHIVE_NAME}" "${ARCHIVE_ROOT}/BUILDINFO" >/dev/null
grep -Fx "artifact_architecture=${ARCHITECTURE}" "${ARCHIVE_ROOT}/BUILDINFO" >/dev/null
grep -Fx "source_version=${VERSION}" "${ARCHIVE_ROOT}/BUILDINFO" >/dev/null
grep -E '^source_commit=[0-9a-f]{40}$' "${ARCHIVE_ROOT}/BUILDINFO" >/dev/null
"${ARCHIVE_ROOT}/gvm-gateway" --help >/dev/null
