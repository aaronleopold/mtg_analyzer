import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import Landing from './pages/Landing';
import Lost from './pages/Lost';

// TODO: use React.lazy to import page components, add suspense placeholders

export default function App() {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<Landing />} />
          <Route path="*" element={<Lost />} />
        </Routes>
      </Layout>
    </BrowserRouter>
  );
}
