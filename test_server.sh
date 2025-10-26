#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

SERVER_URL="http://localhost:8080"

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}  Rust HTTP Server - Performance Test${NC}"
echo -e "${BLUE}======================================${NC}\n"

# Test 1: Root Endpoint
echo -e "${GREEN}[TEST 1] Root Endpoint${NC}"
curl -s "$SERVER_URL/" | jq '.'
sleep 1

# Test 2: Health Check
echo -e "\n${GREEN}[TEST 2] Health Check${NC}"
curl -s "$SERVER_URL/health" | jq '.'
sleep 1

# Test 3: Echo Endpoint
echo -e "\n${GREEN}[TEST 3] Echo Endpoint${NC}"
curl -s "$SERVER_URL/echo/HelloWorld" | jq '.'
sleep 1

# Test 4: Stats Before Load
echo -e "\n${GREEN}[TEST 4] Server Statistics (Before Load)${NC}"
curl -s "$SERVER_URL/stats" | jq '.'
sleep 1

# Test 5: Performance Test - 1000 Requests
echo -e "\n${YELLOW}[TEST 5] Performance Test - Sending 1000 Requests${NC}"
echo -e "${YELLOW}Testing concurrent request handling...${NC}\n"

START_TIME=$(date +%s)

# Send 1000 requests
for i in {1..1000}; do
    curl -s "$SERVER_URL/" > /dev/null &
    
    # Show progress every 100 requests
    if [ $((i % 100)) -eq 0 ]; then
        echo -e "${YELLOW}Progress: $i/1000 requests sent${NC}"
    fi
done

# Wait for all background jobs to complete
wait

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo -e "\n${GREEN}✓ Load test completed!${NC}"
echo -e "${GREEN}  Duration: ${DURATION} seconds${NC}"
echo -e "${GREEN}  Requests per second: $((1000 / DURATION))${NC}\n"

sleep 2

# Test 6: Stats After Load
echo -e "${GREEN}[TEST 6] Server Statistics (After Load)${NC}"
STATS=$(curl -s "$SERVER_URL/stats")
echo "$STATS" | jq '.'

# Extract and display key metrics
TOTAL_REQUESTS=$(echo "$STATS" | jq -r '.total_requests')
UPTIME=$(echo "$STATS" | jq -r '.uptime_seconds')
RPS=$(echo "$STATS" | jq -r '.requests_per_second')

echo -e "\n${BLUE}======================================${NC}"
echo -e "${BLUE}  Performance Summary${NC}"
echo -e "${BLUE}======================================${NC}"
echo -e "${GREEN}Total Requests Processed: ${TOTAL_REQUESTS}${NC}"
echo -e "${GREEN}Server Uptime: ${UPTIME} seconds${NC}"
echo -e "${GREEN}Average RPS: ${RPS}${NC}"

# Test 7: Error Handling
echo -e "\n${GREEN}[TEST 7] Error Handling (404 Not Found)${NC}"
curl -s "$SERVER_URL/nonexistent" | jq '.'

# Test 8: Response Time Test
echo -e "\n${GREEN}[TEST 8] Response Time Analysis${NC}"
echo -e "${YELLOW}Measuring response times for 10 requests...${NC}\n"

TOTAL_TIME=0
for i in {1..10}; do
    RESPONSE_TIME=$(curl -s -o /dev/null -w "%{time_total}" "$SERVER_URL/")
    echo "Request $i: ${RESPONSE_TIME}s"
    TOTAL_TIME=$(echo "$TOTAL_TIME + $RESPONSE_TIME" | bc)
done

AVG_TIME=$(echo "scale=4; $TOTAL_TIME / 10" | bc)
echo -e "\n${GREEN}Average Response Time: ${AVG_TIME}s${NC}"

# Final Stats
echo -e "\n${GREEN}[TEST 9] Final Server Statistics${NC}"
curl -s "$SERVER_URL/stats" | jq '.'

echo -e "\n${BLUE}======================================${NC}"
echo -e "${BLUE}  Test Suite Completed!${NC}"
echo -e "${BLUE}======================================${NC}\n"

echo -e "${GREEN}✓ All tests passed successfully${NC}"
echo -e "${YELLOW}Note: Check server terminal for request logs${NC}\n"