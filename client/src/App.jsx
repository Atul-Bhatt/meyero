import './App.css'
import { APP_ROUTES } from './utils/constants'
import Login from './Components/Login/Login'
import Signup from './Components/Signup/Signup'
import WebSocket from './WebSocket/WebSocket'
import Home from './Pages/Home/Home'
import NotFound from './Pages/NotFound/NotFound'
import { Routes, Route } from 'react-router-dom'
import axios from 'axios';

function App() {
  const token = localStorage.getItem("token");
  if (token) {
    axios.defaults.headers.common["Authorization"] = `Bearer ${token}`;
  }
  return (
    <>
      <Routes>
        <Route path={APP_ROUTES.HOME} element={<Home />} />
        <Route path={APP_ROUTES.LOG_IN} element={<Login />} />
        <Route path={APP_ROUTES.SIGN_UP} element={<Signup />} />
        <Route path={APP_ROUTES.WEBSOCKET} element={<WebSocket />} />
        <Route path='*' element={<NotFound />} />
      </Routes>
    </>
    // <div className="App">
    //   <header className="App-header">
    //     <LoginSignup />
    //     {/* <WebSocket/> */}
    //   </header>
    // </div>
  )
}

export default App
