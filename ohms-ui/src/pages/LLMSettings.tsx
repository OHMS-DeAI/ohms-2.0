import React, { useState } from 'react';
import Card from '../components/Card';
import Input from '../components/Input';
import Button from '../components/Button';

interface ProviderConfig {
  name: string;
  displayName: string;
  baseUrl: string;
  freeRPM: number;
  freeTPM: number;
  defaultModel: string;
}

const DEFAULT_PROVIDERS: ProviderConfig[] = [
  {
    name: 'groq',
    displayName: 'Groq',
    baseUrl: 'https://api.groq.com/openai/v1/chat/completions',
    freeRPM: 30,
    freeTPM: 14400,
    defaultModel: 'llama-3.1-8b-instant',
  },
  {
    name: 'openrouter',
    displayName: 'OpenRouter',
    baseUrl: 'https://openrouter.ai/api/v1/chat/completions',
    freeRPM: 20,
    freeTPM: 10000,
    defaultModel: 'meta-llama/llama-3.1-8b-instruct:free',
  },
];

interface UserAPIKey {
  provider: string;
  apiKey: string;
  added: boolean;
}

const LLMSettings: React.FC = () => {
  const [apiKeys, setApiKeys] = useState<Map<string, UserAPIKey>>(new Map());
  const [testingProvider, setTestingProvider] = useState<string | null>(null);
  const [testResults, setTestResults] = useState<Map<string, { success: boolean; message: string }>>(new Map());

  const handleAddKey = (providerName: string) => {
    const apiKey = window.prompt(`Enter your API key for ${providerName}:`);
    if (apiKey && apiKey.trim()) {
      setApiKeys(prev => new Map(prev).set(providerName, {
        provider: providerName,
        apiKey: apiKey.trim(),
        added: true,
      }));
    }
  };

  const handleRemoveKey = (providerName: string) => {
    if (window.confirm(`Remove API key for ${providerName}?`)) {
      setApiKeys(prev => {
        const newMap = new Map(prev);
        newMap.delete(providerName);
        return newMap;
      });
      setTestResults(prev => {
        const newMap = new Map(prev);
        newMap.delete(providerName);
        return newMap;
      });
    }
  };

  const handleTestKey = async (provider: ProviderConfig) => {
    const userKey = apiKeys.get(provider.name);
    if (!userKey) {
      return;
    }

    setTestingProvider(provider.name);
    setTestResults(prev => new Map(prev).set(provider.name, {
      success: false,
      message: 'Testing...',
    }));

    try {
      const response = await fetch(provider.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${userKey.apiKey}`,
        },
        body: JSON.stringify({
          model: provider.defaultModel,
          messages: [{ role: 'user', content: 'test' }],
          max_tokens: 5,
        }),
      });

      if (response.ok) {
        setTestResults(prev => new Map(prev).set(provider.name, {
          success: true,
          message: 'API key is valid and working!',
        }));
      } else {
        setTestResults(prev => new Map(prev).set(provider.name, {
          success: false,
          message: `Failed: ${response.status} ${response.statusText}`,
        }));
      }
    } catch (error) {
      setTestResults(prev => new Map(prev).set(provider.name, {
        success: false,
        message: `Error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      }));
    } finally {
      setTestingProvider(null);
    }
  };

  return (
    <div className="max-w-4xl mx-auto p-6">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">LLM Provider Settings</h1>
        <p className="text-gray-600">
          Manage your API keys for different LLM providers. Add your own keys for unlimited usage (BYOK - Bring Your Own Key).
        </p>
      </div>

      <Card className="mb-6 bg-blue-50 border-blue-200">
        <div className="flex items-start gap-3">
          <div className="text-blue-600 text-xl">ℹ</div>
          <div>
            <h3 className="font-semibold text-blue-900 mb-1">Default Free Providers</h3>
            <p className="text-blue-800 text-sm">
              OHMS uses multiple free LLM providers with smart rotation to avoid rate limits. 
              You can add your own API keys for unlimited usage and priority access.
            </p>
          </div>
        </div>
      </Card>

      <div className="space-y-4">
        {DEFAULT_PROVIDERS.map(provider => {
          const userKey = apiKeys.get(provider.name);
          const testResult = testResults.get(provider.name);

          return (
            <Card key={provider.name}>
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    <h3 className="text-xl font-bold">{provider.displayName}</h3>
                    {userKey ? (
                      <span className="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm font-medium">
                        Custom Key Active
                      </span>
                    ) : (
                      <span className="px-3 py-1 bg-gray-100 text-gray-800 rounded-full text-sm font-medium">
                        Using Free Tier
                      </span>
                    )}
                  </div>

                  <div className="text-sm text-gray-600 space-y-1 mb-3">
          <p>Model: <span className="font-mono">Managed by provider</span></p>
                    <p>Free Rate Limits: {provider.freeRPM} RPM, {provider.freeTPM.toLocaleString()} TPM</p>
                  </div>

                  {userKey && (
                    <div className="bg-gray-50 p-3 rounded mb-3">
                      <p className="text-sm font-mono text-gray-700 mb-2">
                        API Key: {userKey.apiKey.substring(0, 8)}...{userKey.apiKey.substring(userKey.apiKey.length - 4)}
                      </p>
                      {testResult && (
                        <div className={`text-sm p-2 rounded ${
                          testResult.success ? 'bg-green-50 text-green-800' : 'bg-red-50 text-red-800'
                        }`}>
                          {testResult.message}
                        </div>
                      )}
                    </div>
                  )}

                  <div className="flex gap-2">
                    {userKey ? (
                      <>
                        <Button
                          onClick={() => handleTestKey(provider)}
                          disabled={testingProvider === provider.name}
                          size="sm"
                          variant="secondary"
                        >
                          {testingProvider === provider.name ? 'Testing...' : 'Test API Key'}
                        </Button>
                        <Button
                          onClick={() => handleRemoveKey(provider.name)}
                          size="sm"
                          variant="secondary"
                        >
                          Remove Key
                        </Button>
                      </>
                    ) : (
                      <Button
                        onClick={() => handleAddKey(provider.displayName)}
                        size="sm"
                      >
                        Add Custom API Key
                      </Button>
                    )}
                  </div>
                </div>
              </div>
            </Card>
          );
        })}
      </div>

      <Card className="mt-6 bg-yellow-50 border-yellow-200">
        <div className="flex items-start gap-3">
          <div className="text-yellow-600 text-xl">⚠</div>
          <div>
            <h3 className="font-semibold text-yellow-900 mb-1">Security Note</h3>
            <p className="text-yellow-800 text-sm">
              API keys are stored in your browser's local storage and are never sent to our servers. 
              They are used directly from your browser to call the LLM providers. Keep your keys secure and never share them.
            </p>
          </div>
        </div>
      </Card>
    </div>
  );
};

export default LLMSettings;
