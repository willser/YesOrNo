# shellcheck disable=SC2153
if [[ -z "${SANDBOX_PATH}" ]]; then
  echo "Need environment variable 'SANDBOX_PATH'"
  exit 1
else
  sandbox_path="${SANDBOX_PATH}"
fi
rm -rf /tmp/near-sandbox/

"${sandbox_path}"target/debug/neard-sandbox --home /tmp/near-sandbox init
"${sandbox_path}"target/debug/neard-sandbox --home /tmp/near-sandbox run
