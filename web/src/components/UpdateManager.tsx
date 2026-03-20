'use client';

import { useState, useEffect, useCallback } from 'react';

const POLL_INTERVAL = 30000; // 30 seconds
const IDLE_TIMEOUT = 300000; // 5 minutes

export function UpdateManager() {
  const [currentVersion, setCurrentVersion] = useState<string | null>(null);
  const [newVersion, setNewVersion] = useState<string | null>(null);
  const [isVisible, setIsVisible] = useState(false);
  const [lastActivity, setLastActivity] = useState(Date.now());

  const checkVersion = useCallback(async () => {
    try {
      const res = await fetch('/api/version');
      const data = await res.json();
      
      if (!currentVersion) {
        setCurrentVersion(data.version);
      } else if (data.version !== currentVersion) {
        setNewVersion(data.version);
        setIsVisible(true);
      }
    } catch (err) {
      console.error('Failed to check for updates:', err);
    }
  }, [currentVersion]);

  useEffect(() => {
    checkVersion();
    const interval = setInterval(checkVersion, POLL_INTERVAL);
    
    const handleActivity = () => setLastActivity(Date.now());
    window.addEventListener('mousemove', handleActivity);
    window.addEventListener('keydown', handleActivity);

    return () => {
      clearInterval(interval);
      window.removeEventListener('mousemove', handleActivity);
      window.removeEventListener('keydown', handleActivity);
    };
  }, [checkVersion]);

  useEffect(() => {
    if (newVersion && Date.now() - lastActivity > IDLE_TIMEOUT) {
      // Auto-refresh if idle
      window.location.reload();
    }
  }, [newVersion, lastActivity]);

  if (!isVisible) return null;

  return (
    <div className="update-toast glitch-border">
      <div className="update-content">
        <div className="update-glitch-text" data-text="SYSTEM UPDATE READY">SYSTEM UPDATE READY</div>
        <p className="mono">v{currentVersion} → v{newVersion}</p>
        <div className="update-actions">
          <button onClick={() => window.location.reload()} className="update-btn">
            RELOAD SYSTEM
          </button>
          <button onClick={() => setIsVisible(false)} className="update-close">
            IGNORE
          </button>
        </div>
      </div>
    </div>
  );
}
