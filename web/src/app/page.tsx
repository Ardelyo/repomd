'use client';

import { useState } from 'react';

export default function Home() {
  const [url, setUrl] = useState('');
  const [preset, setPreset] = useState('medium');
  const [tokens, setTokens] = useState(50000);
  const [loading, setLoading] = useState(false);
  const [output, setOutput] = useState('');
  const [stats, setStats] = useState<any>(null);

  const presets = ['light', 'medium', 'aggressive', 'ultra'];

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!url) return;
    
    setLoading(true);
    setOutput('');
    setStats(null);
    
    try {
      // In Phase 3 local routing, this connects to the localhost axum server
      const res = await fetch('http://localhost:8080/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          source: url,
          preset,
          max_tokens: tokens,
        })
      });
      
      if (!res.ok) throw new Error('API Execution Failed');
      
      const data = await res.json();
      setOutput(data.output);
      setStats(data.stats);
    } catch (err: any) {
      setOutput(`[SYSTEM_ERROR]: ${err.message || 'Connection refused or timeout.'}`);
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(output);
  };

  return (
    <main className="container">
      <div style={{ marginBottom: '3rem' }}>
        <h1>Compile Context.</h1>
        <p className="mono" style={{ color: 'var(--color-text-dim)', maxWidth: '600px', lineHeight: 1.6 }}>
          Ingest any repository. AST structural extraction. Smart context knapsack optimization. 
          Generate a single, perfect Markdown payload ready for LLM consumption. 
        </p>
      </div>

      <div className="card">
        <form onSubmit={handleSubmit}>
          <div className="input-group">
            <label>Target Repository URL</label>
            <input 
              type="text" 
              className="input-field" 
              placeholder="https://github.com/user/repo"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              required
              disabled={loading}
            />
          </div>

          <div className="input-group">
            <label>Compression Preset</label>
            <div className="preset-selector">
              {presets.map(p => (
                <button 
                  key={p} 
                  type="button"
                  className={`preset-btn ${preset === p ? 'active' : ''}`}
                  onClick={() => setPreset(p)}
                  disabled={loading}
                >
                  {p}
                </button>
              ))}
            </div>
          </div>

          <div className="input-group" style={{ marginBottom: '2rem' }}>
            <label>Token Budget Limit</label>
            <div className="slider-container">
              <input 
                type="range" 
                min="10000" 
                max="200000" 
                step="5000"
                value={tokens}
                onChange={(e) => setTokens(Number(e.target.value))}
                style={{ flexGrow: 1, accentColor: 'var(--color-primary)' }}
                disabled={loading}
              />
              <span className="slider-value">{tokens.toLocaleString()}</span>
            </div>
          </div>

          <button type="submit" className="submit-btn" disabled={loading || !url}>
            {loading ? 'EXECUTING PARSER...' : 'GENERATE REPO.MD'}
          </button>
        </form>
      </div>

      {(output || loading) && (
        <div className={`code-window ${loading ? 'flicker' : ''}`}>
          {stats && !loading && (
            <div className="stat-grid">
              <div className="stat-box">
                <span className="stat-label">Output Tokens</span>
                <span className="stat-val">{stats.output_tokens.toLocaleString()}</span>
              </div>
              <div className="stat-box">
                <span className="stat-label">Execution Time</span>
                <span className="stat-val">{stats.processing_time_ms}ms</span>
              </div>
              <div className="stat-box">
                <span className="stat-label">Action</span>
                <button 
                  type="button" 
                  className="preset-btn active" 
                  style={{ width: 'fit-content', marginTop: '0.25rem' }}
                  onClick={copyToClipboard}
                >
                  COPY PAYLOAD
                </button>
              </div>
            </div>
          )}
          
          <div className="code-content">
            {loading ? 'INITIALIZING CONNECTION TO REPOMD CORE...\nESTABLISHING AST PARSER LIMITS...\nWAITING FOR PAYLOAD...' : output}
          </div>
        </div>
      )}
    </main>
  );
}
