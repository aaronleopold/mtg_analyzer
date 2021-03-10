import 'core-js';
import './css/styles.css';
import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

// ReactDOM.unstable_createRoot(document.getElementById('root')!).render(<App />);

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);
