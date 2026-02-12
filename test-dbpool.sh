#!/usr/bin/env bash

BASE="http://localhost:4040"
OUT="agent-docs/test-dbpool.md"
PASS=0
FAIL=0

mkdir -p agent-docs

# Run a test: $1=label, $2=method, $3=url, $4=expected HTTP code, $5=body (optional)
run_test() {
  local label="$1" method="$2" url="$3" expect="$4" body="${5:-}"

  local curl_args=(-s -w '\n%{http_code}' -X "$method")
  if [[ -n "$body" ]]; then
    curl_args+=(-H "Content-Type: application/json" -d "$body")
  fi

  local raw
  raw=$(curl "${curl_args[@]}" "$url" 2>/dev/null)
  local code
  code=$(echo "$raw" | tail -1)
  local response
  response=$(echo "$raw" | sed '$d')

  local pretty
  pretty=$(echo "$response" | jq . 2>/dev/null || echo "$response")

  local status="PASS"
  if [[ "$code" != "$expect" ]]; then
    status="FAIL"
    FAIL=$((FAIL + 1))
  else
    PASS=$((PASS + 1))
  fi

  echo "## $label"
  echo ""
  echo "**$status** — $method $url — HTTP $code (expected $expect)"
  echo ""
  echo '```json'
  echo "$pretty"
  echo '```'
  echo ""

  # Return the raw response for callers to parse
  LAST_RESPONSE="$response"
}

{
  echo "# dbPool Test Results"
  echo ""
  echo "Run: $(date -Iseconds)"
  echo ""

  # 1. Health
  run_test "1. Health Check" GET "$BASE/health" 200

  # 2. List (empty)
  run_test "2. List Entries (empty)" GET "$BASE/api/whitelist" 200

  # 3. Create entry
  run_test "3. Create Entry" POST "$BASE/api/whitelist" 200 \
    '{"phone_number":"555-234-5678","name":"Test User"}'
  ENTRY_ID=$(echo "$LAST_RESPONSE" | jq -r '.data.id // empty')

  # 4. Create with optional fields
  run_test "4. Create Entry (with reason + expires_at)" POST "$BASE/api/whitelist" 200 \
    '{"phone_number":"(212) 867-5309","name":"Jenny","reason":"VIP","expires_at":"2026-12-31T00:00:00Z","is_permanent":false}'
  ENTRY2_ID=$(echo "$LAST_RESPONSE" | jq -r '.data.id // empty')

  # 5. List (populated)
  run_test "5. List Entries (populated)" GET "$BASE/api/whitelist" 200

  # 6. Get by ID
  if [[ -n "${ENTRY_ID:-}" ]]; then
    run_test "6. Get Entry by ID" GET "$BASE/api/whitelist/$ENTRY_ID" 200
  else
    echo "## 6. Get Entry by ID"
    echo ""
    echo "**SKIP** — no entry ID from step 3"
    echo ""
    FAIL=$((FAIL + 1))
  fi

  # 7. Delete first entry
  if [[ -n "${ENTRY_ID:-}" ]]; then
    run_test "7. Delete Entry" DELETE "$BASE/api/whitelist/$ENTRY_ID" 200
  else
    echo "## 7. Delete Entry"
    echo ""
    echo "**SKIP** — no entry ID from step 3"
    echo ""
    FAIL=$((FAIL + 1))
  fi

  # 8. Delete second entry
  if [[ -n "${ENTRY2_ID:-}" ]]; then
    run_test "8. Delete Second Entry" DELETE "$BASE/api/whitelist/$ENTRY2_ID" 200
  else
    echo "## 8. Delete Second Entry"
    echo ""
    echo "**SKIP** — no entry ID from step 4"
    echo ""
    FAIL=$((FAIL + 1))
  fi

  # 9. List (should be empty again)
  run_test "9. List Entries (after deletion)" GET "$BASE/api/whitelist" 200

  # 10. Validation: phone too short
  run_test "10. Validation: Bad Phone (too short)" POST "$BASE/api/whitelist" 400 \
    '{"phone_number":"123","name":"Bad"}'

  # 11. Validation: area code starts with 0
  run_test "11. Validation: Area Code Starts with 0" POST "$BASE/api/whitelist" 400 \
    '{"phone_number":"055-234-5678","name":"Bad"}'

  # 12. Not found: get
  run_test "12. Not Found: Get" GET "$BASE/api/whitelist/00000000-0000-0000-0000-000000000000" 404

  # 13. Not found: delete
  run_test "13. Not Found: Delete" DELETE "$BASE/api/whitelist/00000000-0000-0000-0000-000000000000" 404

  echo "---"
  echo ""
  echo "**Results: $PASS passed, $FAIL failed out of $((PASS + FAIL)) tests**"

} > "$OUT"

echo "Tests complete: $PASS passed, $FAIL failed. Results written to $OUT"
