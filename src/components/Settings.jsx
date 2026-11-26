import React, { useState } from 'react';

export default function Settings({ onClose }) {
  const [darkMode, setDarkMode] = useState(false);
  const [showUpdates, setShowUpdates] = useState(true);

  return (
    <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h2>设置</h2>
        <button onClick={onClose}>关闭</button>
      </div>
      
      <div style={{ marginTop: '20px' }}>
        <h3>界面</h3>
        <label style={{ display: 'block', margin: '10px 0' }}>
          <input
            type="checkbox"
            checked={darkMode}
            onChange={(e) => setDarkMode(e.target.checked)}
          />
          深色模式
        </label>
      </div>

      <div style={{ marginTop: '20px' }}>
        <h3>通知</h3>
        <label style={{ display: 'block', margin: '10px 0' }}>
          <input
            type="checkbox"
            checked={showUpdates}
            onChange={(e) => setShowUpdates(e.target.checked)}
          />
          显示更新通知
        </label>
      </div>

      <div style={{ marginTop: '20px' }}>
        <h3>关于</h3>
        <p>NovaKit v0.1.0</p>
        <p>离线工具箱，无广告，无遥测。</p>
      </div>
    </div>
  );
}
