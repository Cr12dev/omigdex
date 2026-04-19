import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ThemeProvider, useTheme } from './components/ThemeToggle';
import { ToastProvider, useToast } from './components/Toast';
import { DownloadForm } from './components/DownloadForm';
import { DownloadQueue } from './components/DownloadQueue';
import { DownloadHistory } from './components/DownloadHistory';
import './App.css';

interface DownloadInfo {
  id: string;
  url: string;
  title: string;
  platform: string;
  format: string;
  quality: string;
  status: string;
  progress: number;
  file_path: string | null;
  created_at: string;
}

function AppContent() {
  const { theme, toggleTheme } = useTheme();
  const { showToast } = useToast();
  const [downloads, setDownloads] = useState<DownloadInfo[]>([]);
  const [history, setHistory] = useState<DownloadInfo[]>([]);
  const [activeTab, setActiveTab] = useState<'queue' | 'history'>('queue');

  useEffect(() => {
    loadDownloads();
    loadHistory();
    
    const interval = setInterval(() => {
      loadDownloads();
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  const loadDownloads = async () => {
    try {
      const data = await invoke<DownloadInfo[]>('get_all_downloads');
      setDownloads(data);
    } catch (error) {
      console.error('Failed to load downloads:', error);
    }
  };

  const loadHistory = async () => {
    try {
      const data = await invoke<DownloadInfo[]>('get_history');
      setHistory(data);
    } catch (error) {
      console.error('Failed to load history:', error);
    }
  };

  const handleDownloadStart = async (url: string, format: string, quality: string) => {
    try {
      showToast('Starting download...', 'info');
      
      const id = await invoke<string>('download_video', { url, format, quality });
      
      const videoInfo = await invoke<[string, string]>('get_video_info', { url });
      const newDownload: DownloadInfo = {
        id,
        url,
        title: videoInfo[0],
        platform: videoInfo[1],
        format,
        quality,
        status: 'pending',
        progress: 0,
        file_path: null,
        created_at: new Date().toISOString(),
      };
      
      setDownloads(prev => [...prev, newDownload]);
      showToast(`Download started: ${videoInfo[0]}`, 'success');
    } catch (error) {
      console.error('Failed to start download:', error);
      showToast(`Failed to start download: ${error}`, 'error');
    }
  };

  const handleCancelDownload = async (id: string) => {
    try {
      await invoke('cancel_download', { id });
      loadDownloads();
      showToast('Download cancelled', 'info');
    } catch (error) {
      console.error('Failed to cancel download:', error);
      showToast('Failed to cancel download', 'error');
    }
  };

  const handleClearCompleted = async () => {
    try {
      await invoke('clear_completed');
      loadDownloads();
      showToast('Completed downloads cleared', 'info');
    } catch (error) {
      console.error('Failed to clear completed:', error);
      showToast('Failed to clear completed downloads', 'error');
    }
  };

  const handleRefreshHistory = () => {
    loadHistory();
    showToast('History refreshed', 'info');
  };

  return (
    <div className={`app ${theme}`}>
      <header className="app-header">
        <h1>Omigdex</h1>
        <button onClick={toggleTheme} className="theme-toggle">
          {theme === 'light' ? 'Dark' : 'Light'}
        </button>
      </header>

      <main className="app-main">
        <DownloadForm onDownloadStart={handleDownloadStart} />

        <div className="tabs">
          <button
            className={`tab ${activeTab === 'queue' ? 'active' : ''}`}
            onClick={() => setActiveTab('queue')}
          >
            Queue ({downloads.filter(d => d.status !== 'completed' && d.status !== 'failed' && d.status !== 'cancelled').length})
          </button>
          <button
            className={`tab ${activeTab === 'history' ? 'active' : ''}`}
            onClick={() => setActiveTab('history')}
          >
            History ({history.length})
          </button>
        </div>

        {activeTab === 'queue' ? (
          <DownloadQueue
            downloads={downloads}
            onCancel={handleCancelDownload}
            onClearCompleted={handleClearCompleted}
          />
        ) : (
          <DownloadHistory
            history={history}
            onRefresh={handleRefreshHistory}
          />
        )}
      </main>
    </div>
  );
}

function App() {
  return (
    <ThemeProvider>
      <ToastProvider>
        <AppContent />
      </ToastProvider>
    </ThemeProvider>
  );
}

export default App;
