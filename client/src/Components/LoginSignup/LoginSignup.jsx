import './LoginSignup.css'
import { API_ROUTES, APP_ROUTES } from '../../utils/constants'

import google_logo from '../Assets/google_logo.png';
import apple_logo from '../Assets/apple_logo.svg';
import login_background from '../Assets/login_background.jpg';
import axios from 'axios';
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom'

const LoginSignup = () => {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const navigate = useNavigate();

	useEffect(()=>{
        if (localStorage.getItem("token") != undefined) {
            navigate(APP_ROUTES.HOME)
        }
	},[])

    const handleSubmit = async (event) => {
        event.preventDefault();

        try {
            const response = await axios.post(API_ROUTES.LOG_IN, {
                username,
                password
            });

			const token = response.data.data.token
            const userId = response.data.data.user_id
            const userName = response.data.data.user_name
			localStorage.setItem("token", token)
            localStorage.setItem("userId", userId)
            localStorage.setItem("userName", userName)
			axios.defaults.headers.common["Authorization"] = `Bearer ${token}`
            navigate(APP_ROUTES.HOME)
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
            <div className="other-login-button">
                <div>
                    <img src={google_logo} alt="google_logo"></img>
                </div>
                <div>
                    <span>Continue with Google</span>
                </div>
            </div>
            <br/>
            <div className="other-login-button">
                <img src={apple_logo} alt="apple_logo" />
                <span>Continue with Apple</span>
            </div>
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
