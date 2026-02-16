#!/usr/bin/env bash
set -euo pipefail

BASE="http://localhost:4040"
PASS=0
FAIL=0

check() {
  local desc="$1" expected="$2" actual="$3"
  if [[ "$actual" == *"$expected"* ]]; then
    echo "  PASS: $desc"
    ((PASS++))
  else
    echo "  FAIL: $desc (expected '$expected')"
    echo "    got: $actual"
    ((FAIL++))
  fi
}

echo "=== Booking API Tests ==="
echo

# 1. Create booking
echo "[1] Create booking"
RES=$(curl -s -X POST "$BASE/api/bookings" \
  -H 'Content-Type: application/json' \
  -d '{"caller_name":"Test User","caller_phone":"555-234-5678"}')
check "success" '"success":true' "$RES"
check "status pending" '"status":"pending"' "$RES"
ID1=$(echo "$RES" | jq -r '.data.id')
check "has id" "-" "$ID1"
echo

# 2. Create booking with email + reason
echo "[2] Create booking with email and reason"
RES=$(curl -s -X POST "$BASE/api/bookings" \
  -H 'Content-Type: application/json' \
  -d '{"caller_name":"Jane Doe","caller_phone":"555-987-6543","caller_email":"jane@example.com","call_reason":"Schedule repair"}')
check "success" '"success":true' "$RES"
check "has email" '"caller_email":"jane@example.com"' "$RES"
check "has reason" '"call_reason":"Schedule repair"' "$RES"
ID2=$(echo "$RES" | jq -r '.data.id')
echo

# 3. List all bookings
echo "[3] List all bookings"
RES=$(curl -s "$BASE/api/bookings")
check "success" '"success":true' "$RES"
COUNT=$(echo "$RES" | jq '.data | length')
check "at least 2 bookings" "true" "$([ "$COUNT" -ge 2 ] && echo true || echo false)"
echo

# 4. Filter by status
echo "[4] Filter by status=pending"
RES=$(curl -s "$BASE/api/bookings?status=pending")
check "success" '"success":true' "$RES"
ALL_PENDING=$(echo "$RES" | jq '[.data[].status] | all(. == "pending")')
check "all are pending" "true" "$ALL_PENDING"
echo

# 5. Get single booking
echo "[5] Get single booking"
RES=$(curl -s "$BASE/api/bookings/$ID1")
check "success" '"success":true' "$RES"
check "correct id" "$ID1" "$RES"
echo

# 6. Approve booking -> auto-creates whitelist entry
echo "[6] Approve booking"
RES=$(curl -s -X PATCH "$BASE/api/bookings/$ID1" \
  -H 'Content-Type: application/json' \
  -d '{"status":"approved"}')
check "success" '"success":true' "$RES"
check "status approved" '"status":"approved"' "$RES"
check "has resolved_at" '"resolved_at"' "$RES"
check "has whitelist_entry_id" '"whitelist_entry_id"' "$RES"
WL_ID=$(echo "$RES" | jq -r '.data.whitelist_entry_id')
check "whitelist_entry_id not null" "true" "$([ "$WL_ID" != "null" ] && echo true || echo false)"
echo

# 7. Verify whitelist entry was created
echo "[7] Verify whitelist entry exists"
RES=$(curl -s "$BASE/api/whitelist/$WL_ID")
check "success" '"success":true' "$RES"
check "name matches" '"name":"Test User"' "$RES"
echo

# 8. Deny booking
echo "[8] Deny booking"
RES=$(curl -s -X PATCH "$BASE/api/bookings/$ID2" \
  -H 'Content-Type: application/json' \
  -d '{"status":"denied"}')
check "success" '"success":true' "$RES"
check "status denied" '"status":"denied"' "$RES"
check "has resolved_at" '"resolved_at"' "$RES"
check "no whitelist_entry_id" '"whitelist_entry_id":null' "$RES"
echo

# 9. Double-resolve should fail
echo "[9] Double-resolve (should fail)"
RES=$(curl -s -X PATCH "$BASE/api/bookings/$ID1" \
  -H 'Content-Type: application/json' \
  -d '{"status":"denied"}')
check "fails" '"success":false' "$RES"
check "already resolved msg" 'already resolved' "$RES"
echo

# 10. Bad phone number
echo "[10] Bad phone number (should fail)"
RES=$(curl -s -X POST "$BASE/api/bookings" \
  -H 'Content-Type: application/json' \
  -d '{"caller_name":"Bad Phone","caller_phone":"123"}')
check "fails" '"success":false' "$RES"
echo

# 11. Bad email
echo "[11] Bad email (should fail)"
RES=$(curl -s -X POST "$BASE/api/bookings" \
  -H 'Content-Type: application/json' \
  -d '{"caller_name":"Bad Email","caller_phone":"555-234-5678","caller_email":"notanemail"}')
check "fails" '"success":false' "$RES"
check "invalid email msg" 'invalid email' "$RES"
echo

# 12. Nonexistent booking
echo "[12] Get nonexistent booking (should 404)"
RES=$(curl -s "$BASE/api/bookings/00000000-0000-0000-0000-000000000000")
check "fails" '"success":false' "$RES"
echo

# Summary
echo "=== Results: $PASS passed, $FAIL failed ==="
exit "$FAIL"
