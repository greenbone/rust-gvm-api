#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: build-binary-archive.sh --version <semver> --architecture <amd64|arm64> [--output-dir <dir>]
EOF
}

VERSION=""
ARCHITECTURE=""
OUTPUT_DIR="dist/binaries"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    --architecture)
      ARCHITECTURE="${2:-}"
      shift 2
      ;;
    --output-dir)
      OUTPUT_DIR="${2:-}"
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

if [[ -z "${VERSION}" || -z "${ARCHITECTURE}" || -z "${OUTPUT_DIR}" ]]; then
  usage >&2
  exit 1
fi

if [[ ! "${VERSION}" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
  echo "version must be semantic-version shaped: ${VERSION}" >&2
  exit 1
fi

ROOT_DIR="$(git rev-parse --show-toplevel)"
cd "${ROOT_DIR}"

case "$(uname -m)" in
  x86_64|amd64)
    HOST_ARCHITECTURE="amd64"
    ;;
  aarch64|arm64)
    HOST_ARCHITECTURE="arm64"
    ;;
  *)
    echo "unsupported Linux host architecture: $(uname -m)" >&2
    exit 1
    ;;
esac

case "${ARCHITECTURE}" in
  amd64|arm64)
    ;;
  *)
    echo "unsupported archive architecture: ${ARCHITECTURE}" >&2
    exit 1
    ;;
esac

if [[ "${ARCHITECTURE}" != "${HOST_ARCHITECTURE}" ]]; then
  echo "refusing to label ${HOST_ARCHITECTURE} binary as ${ARCHITECTURE}" >&2
  exit 1
fi

case "${OUTPUT_DIR}" in
  /*)
    OUTPUT_DIR_ABS="$(realpath -m "${OUTPUT_DIR}")"
    ;;
  *)
    OUTPUT_DIR_ABS="$(realpath -m "${ROOT_DIR}/${OUTPUT_DIR}")"
    ;;
esac

case "${OUTPUT_DIR_ABS}" in
  "${ROOT_DIR}/dist/"*)
    ;;
  *)
    echo "archive output directory must be under dist/: ${OUTPUT_DIR}" >&2
    exit 1
    ;;
esac

if [[ "${OUTPUT_DIR_ABS}" == "${ROOT_DIR}/dist" ]]; then
  echo "archive output directory must not be dist/ itself" >&2
  exit 1
fi

BINARY="${ROOT_DIR}/target/release/gvm-gateway"
if [[ ! -x "${BINARY}" ]]; then
  echo "release binary missing: target/release/gvm-gateway" >&2
  exit 1
fi

ARCHIVE_ROOT_NAME="gvm-gateway-${VERSION}-linux-${ARCHITECTURE}"
STAGING_DIR="${ROOT_DIR}/dist/binary-archive"
ARCHIVE_ROOT="${STAGING_DIR}/${ARCHIVE_ROOT_NAME}"
ARCHIVE_NAME="gvm-gateway_${VERSION}_linux_${ARCHITECTURE}.tar.gz"
ARCHIVE_PATH="${OUTPUT_DIR_ABS}/${ARCHIVE_NAME}"
SOURCE_EPOCH="$(git show -s --format=%ct HEAD)"
BUILT_AT="$(date -u -d "@${SOURCE_EPOCH}" +%Y-%m-%dT%H:%M:%SZ)"

rm -rf "${ARCHIVE_ROOT}"
rm -f "${ARCHIVE_PATH}" "${ARCHIVE_PATH}.sha256"
mkdir -p "${ARCHIVE_ROOT}" "${OUTPUT_DIR_ABS}"

install -m 0755 "${BINARY}" "${ARCHIVE_ROOT}/gvm-gateway"
install -m 0644 packaging/gvm-gateway.toml "${ARCHIVE_ROOT}/gvm-gateway.toml.example"
install -m 0644 README.md "${ARCHIVE_ROOT}/README.md"
install -m 0644 LICENSE "${ARCHIVE_ROOT}/LICENSE"

cat > "${ARCHIVE_ROOT}/BUILDINFO" <<EOF
artifact_name=${ARCHIVE_NAME}
artifact_architecture=${ARCHITECTURE}
source_version=${VERSION}
source_commit=$(git rev-parse HEAD)
built_at=${BUILT_AT}
EOF

(
  cd "${STAGING_DIR}"
  tar \
    --sort=name \
    --mtime="@${SOURCE_EPOCH}" \
    --owner=0 \
    --group=0 \
    --numeric-owner \
    -cf - \
    "${ARCHIVE_ROOT_NAME}" | gzip -n > "${ARCHIVE_PATH}"
)

(
  cd "${OUTPUT_DIR_ABS}"
  sha256sum "${ARCHIVE_NAME}" > "${ARCHIVE_NAME}.sha256"
)

printf '%s\n' "${ARCHIVE_PATH}"
