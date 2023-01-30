import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import {
  BrowserRouter,
  Routes,
  Route,
  Navigate
} from "react-router-dom";
import Login from './auth/login';
import Callback from './auth/callback';

const root = ReactDOM.createRoot(document.getElementById('root'));

const PrivateRoute = ({...props}) => {
    const auth = localStorage.getItem('token');

    return auth ? <App {...props} /> : <Navigate to="/admin/login" replace />;
}

root.render(
  <BrowserRouter>
    <Routes>
      <Route path="/admin/">
        <Route path="login" element={<Login />} />
        <Route path="callback" element={<Callback />} />
        <Route index element={<PrivateRoute />} />
      </Route>
    </Routes>
  </BrowserRouter>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
