import logo from './logo.svg';
import './App.css';
import LoginSignup from './Components/LoginSignup/LoginSignup';
import WebSocket from './WebSocket/WebSocket'

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <LoginSignup /> 
        {/* <WebSocket/> */}
      </header>
    </div>
  );
}

export default App;
