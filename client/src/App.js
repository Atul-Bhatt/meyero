import logo from './logo.svg';
import './App.css';
import LoginSignup from './Components/LoginSignup/LoginSignup';
import WebSocket from './WebSocket/WebSocket'
import Home from './Pages/Home/Home'
import NotFound from './Pages/NotFound/NotFound'
import {Routes, Route} from 'react-router-dom'

function App() {
  return (
    <>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<LoginSignup />} />
        <Route path="/websocket" element={<WebSocket />} />
        <Route path="*" element={<NotFound />} />
      </Routes>
    </>
    // <div className="App">
    //   <header className="App-header">
    //     <LoginSignup /> 
    //     {/* <WebSocket/> */}
    //   </header>
    // </div>
  );
}

export default App;
