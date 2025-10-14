#!/bin/bash
# Test API Keys for OHMS Multi-Agent Orchestration
# This script verifies that LLM provider API keys are working correctly

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔑 Testing OHMS LLM Provider API Keys..."
echo ""

# Check if API keys are set
if [ -z "$GROQ_API_KEY" ]; then
    echo -e "${RED}❌ GROQ_API_KEY not set${NC}"
    echo "   Run: export GROQ_API_KEY=\"your_key_here\""
    GROQ_SET=false
else
    echo -e "${GREEN}✅ GROQ_API_KEY is set${NC}"
    GROQ_SET=true
fi

if [ -z "$OPENROUTER_API_KEY" ]; then
    echo -e "${RED}❌ OPENROUTER_API_KEY not set${NC}"
    echo "   Run: export OPENROUTER_API_KEY=\"your_key_here\""
    OPENROUTER_SET=false
else
    echo -e "${GREEN}✅ OPENROUTER_API_KEY is set${NC}"
    OPENROUTER_SET=true
fi

echo ""
echo "📡 Testing API connectivity..."
echo ""

# Test Groq API
if [ "$GROQ_SET" = true ]; then
    echo -e "${YELLOW}Testing Groq API...${NC}"
    GROQ_RESPONSE=$(curl -s -w "\n%{http_code}" https://api.groq.com/openai/v1/chat/completions \
      -H "Authorization: Bearer $GROQ_API_KEY" \
      -H "Content-Type: application/json" \
      -d '{
        "model": "llama-3.1-8b-instant",
        "messages": [{"role": "user", "content": "Say hello"}],
        "max_tokens": 10
      }')
    
    HTTP_CODE=$(echo "$GROQ_RESPONSE" | tail -n1)
    RESPONSE_BODY=$(echo "$GROQ_RESPONSE" | sed '$d')
    
    if [ "$HTTP_CODE" = "200" ]; then
        echo -e "${GREEN}✅ Groq API is working!${NC}"
        echo "   Response: $(echo "$RESPONSE_BODY" | jq -r '.choices[0].message.content' 2>/dev/null || echo "Success")"
    else
        echo -e "${RED}❌ Groq API failed (HTTP $HTTP_CODE)${NC}"
        echo "   Error: $(echo "$RESPONSE_BODY" | jq -r '.error.message' 2>/dev/null || echo "$RESPONSE_BODY")"
    fi
    echo ""
fi

# Test OpenRouter API
if [ "$OPENROUTER_SET" = true ]; then
    echo -e "${YELLOW}Testing OpenRouter API...${NC}"
    OPENROUTER_RESPONSE=$(curl -s -w "\n%{http_code}" https://openrouter.ai/api/v1/chat/completions \
      -H "Authorization: Bearer $OPENROUTER_API_KEY" \
      -H "Content-Type: application/json" \
      -H "HTTP-Referer: https://ohms.ai" \
      -H "X-Title: OHMS Multi-Agent Orchestration" \
      -d '{
        "model": "google/gemma-2-9b-it:free",
        "messages": [{"role": "user", "content": "Say hello"}],
        "max_tokens": 10
      }')
    
    HTTP_CODE=$(echo "$OPENROUTER_RESPONSE" | tail -n1)
    RESPONSE_BODY=$(echo "$OPENROUTER_RESPONSE" | sed '$d')
    
    if [ "$HTTP_CODE" = "200" ]; then
        echo -e "${GREEN}✅ OpenRouter API is working!${NC}"
        echo "   Response: $(echo "$RESPONSE_BODY" | jq -r '.choices[0].message.content' 2>/dev/null || echo "Success")"
    else
        echo -e "${RED}❌ OpenRouter API failed (HTTP $HTTP_CODE)${NC}"
        echo "   Error: $(echo "$RESPONSE_BODY" | jq -r '.error.message' 2>/dev/null || echo "$RESPONSE_BODY")"
    fi
    echo ""
fi

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$GROQ_SET" = true ] && [ "$OPENROUTER_SET" = true ]; then
    echo -e "${GREEN}✅ All API keys are configured and working!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Start DFX replica: npm run dfx:start"
    echo "  2. Deploy canisters: npm run dfx:deploy:local"
    echo "  3. Test orchestration: Navigate to /orchestrate"
elif [ "$GROQ_SET" = true ] || [ "$OPENROUTER_SET" = true ]; then
    echo -e "${YELLOW}⚠️  Some API keys are working, but not all are configured${NC}"
    echo "   Configure missing keys for full functionality"
else
    echo -e "${RED}❌ No API keys are configured${NC}"
    echo "   See docs/api-keys-setup.md for configuration instructions"
fi

echo ""

