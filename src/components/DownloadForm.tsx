import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface DownloadFormProps {
  onDownloadStart: (url: string, format: string, quality: string) => void;
}

export const DownloadForm: React.FC<DownloadFormProps> = ({ onDownloadStart }) => {
  const [url, setUrl] = useState('');
  const [format, setFormat] = useState('mp4');
  const [quality, setQuality] = useState('high');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!url.trim()) {
      setError('Please enter a URL');
      return;
    }

    setIsLoading(true);
    
    try {
      await invoke('check_yt_dlp');
      onDownloadStart(url, format, quality);
      setUrl('');
    } catch (err) {
      setError('yt-dlp is not installed. Please install it from https://github.com/yt-dlp/yt-dlp');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="download-form">
      <div className="form-group">
        <input
          type="text"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="Paste video URL (YouTube, Instagram, TikTok, Pinterest)"
          className="url-input"
          disabled={isLoading}
        />
      </div>
      
      <div className="form-row">
        <div className="form-group">
          <label htmlFor="format">Format</label>
          <select
            id="format"
            value={format}
            onChange={(e) => setFormat(e.target.value)}
            className="select-input"
            disabled={isLoading}
          >
            <option value="mp4">MP4 (Video)</option>
            <option value="mp3">MP3 (Audio)</option>
          </select>
        </div>
        
        <div className="form-group">
          <label htmlFor="quality">Quality</label>
          <select
            id="quality"
            value={quality}
            onChange={(e) => setQuality(e.target.value)}
            className="select-input"
            disabled={isLoading}
          >
            <option value="best">Best</option>
            <option value="high">High (1080p)</option>
            <option value="medium">Medium (720p)</option>
            <option value="low">Low</option>
          </select>
        </div>
      </div>

      {error && <div className="error-message">{error}</div>}
      
      <button type="submit" className="download-button" disabled={isLoading}>
        {isLoading ? 'Processing...' : 'Download'}
      </button>
    </form>
  );
};
