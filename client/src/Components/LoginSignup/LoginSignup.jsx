import './LoginSignup.css'

import google_logo from '../Assets/google_logo.png';
import apple_logo from '../Assets/apple_logo.png';
import login_background from '../Assets/login_background.jpg';
import axios from 'axios';
import { useState } from 'react';

const LoginSignup = () => {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const url = 'http://localhost:8081/user/login';

    const handleSubmit = (event) => {
        event.preventDefault();

        try {
            const response = axios.post(url, {
                username,
                password
            });

            console.log(response.data);
        } catch(error) {
            console.log(error);
        }
    };

    return (
        <div className="main-container">
        <div className="left-container">
            <img src={login_background} alt="" className="background-image" />
        </div>
        <div className='right-container'>
        <div className="header">
            <div className="text">Log in</div>
        </div>
        <div className="other-logins">
            <button className="other-login-button">
                <img src={google_logo} alt="google_logo"></img>
                <span>Continue with Google</span>
            </button>
            <br/>
            <button className="other-login-button">
                <img src={apple_logo} alt="apple_logo" />
                <span>Continue with Apple</span>
            </button>
        </div>
        <div className="line-container">
            <p><span>OR</span></p>
        </div> 
        <div className="inputs">
            <form onSubmit={handleSubmit}>
            <div className="input">
                <input type="text"
                    placeholder="Username"
                    onChange={(event) => setUsername(event.target.value)}/>
            </div>
            <div className="input">
                <input type="password"
                    placeholder="Password"
                    onChange={(event) => setPassword(event.target.value)}/>
            </div>
            <div className="submit-container">
                <button type="submit">LOG IN</button>
            </div>
            </form>
        </div>
        </div>
        </div>
    );
}

export default LoginSignup;