import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ThemeProvider, useTheme } from './components/ThemeToggle';
import { ToastProvider, useToast } from './components/Toast';
import { DownloadForm } from './components/DownloadForm';
import { DownloadQueue } from './components/DownloadQueue';
import { DownloadHistory } from './components/DownloadHistory';
import './App.css';
import { Button } from './components/ui/button';

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
    <div className={`min-h-screen flex flex-col transition-colors duration-300 ${theme === 'dark' ? 'bg-gray-900 text-white' : 'bg-white text-black'}`}>
      <header className="flex justify-between items-center px-8 py-4 border-b border-gray-200 dark:border-gray-700">
        <h1 className="text-2xl font-semibold">Omigdex</h1>
        <button 
          onClick={toggleTheme} 
          className="px-4 py-2 border border-gray-200 dark:border-gray-700 rounded-lg cursor-pointer text-sm font-medium transition-all hover:bg-gray-100 dark:hover:bg-gray-800"
        >
          {theme === 'light' ? 'Dark' : 'Light'}
        </button>
      </header>

      <main className="flex-1 p-8 max-w-4xl mx-auto w-full">
        <DownloadForm onDownloadStart={handleDownloadStart} />

        <div className="flex gap-2 mb-6 border-b border-gray-200 dark:border-gray-700">
          <button
            className={`px-6 py-3 border-none bg-none text-sm font-medium cursor-pointer border-b-2 transition-all ${activeTab === 'queue' ? 'text-black dark:text-white border-black dark:border-white' : 'text-gray-600 dark:text-gray-400 border-transparent hover:text-black dark:hover:text-white'}`}
            onClick={() => setActiveTab('queue')}
          >
            Queue ({downloads.filter(d => d.status !== 'completed' && d.status !== 'failed' && d.status !== 'cancelled').length})
          </button>
          <button
            className={`px-6 py-3 border-none bg-none text-sm font-medium cursor-pointer border-b-2 transition-all ${activeTab === 'history' ? 'text-black dark:text-white border-black dark:border-white' : 'text-gray-600 dark:text-gray-400 border-transparent hover:text-black dark:hover:text-white'}`}
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
        <div className="mt-8">
          <div className="flex justify-center items-center py-4">
            <Button variant="outline"><a href="https://github.com/Cr12dev/omigdex" target="_blank">Github</a></Button>
          </div>
        </div>
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
