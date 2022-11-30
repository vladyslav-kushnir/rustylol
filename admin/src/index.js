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

const PrivateRoute = ({ ...props }) => {
  if (localStorage.getItem('token')) {
    return <Route { ...props } />;
  }

  props.history.push("/admin/login")
};

root.render(
  <BrowserRouter>
    <Routes>
      <Route path="/admin/login" element={<Login />} />
      <Route path="/admin/callback" element={<Callback />} />
      <PrivateRoute path="/admin/" element={<App />} />
    </Routes>
  </BrowserRouter>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
