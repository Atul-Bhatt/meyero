import './LoginSignup.css'

import google_logo from '../Assets/google_logo.png'
import apple_logo from '../Assets/apple_logo.png'
import login_background from '../Assets/login_background.jpg'
import axios from 'axios'
import React from 'react'
import { Link } from 'react-router-dom'
class LoginSignup extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            username: "",
            password: ""
        };
    }

    handleOnChange = e => {
        this.setState({
            [e.target.name]: e.target.value
        })
    }

    handleSubmit = e => {
        e.preventDefault();
        // axios.post("http://localhost:8081/user/login", {
        //     id: "9eb564d2-82a5-40f3-8fd8-9a0f52676c49", // Not actually used
        //     username: this.state.username,
        //     password: this.state.password
        // })
        // .then(reponse => {
        //     console.log(reponse);
        //     alert('login success')
        //     navigate('/')
        // })
        // .catch(error => {
        //     console.log(error);
        //     alert('login failed')
        // });

        // this.setState({
        //     username: "",
        //     password: ""
        // });

        try {
            const response = axios.post(
                "http://localhost:8081/user/login",
                JSON.stringify(this.state),
                {
                    headers: {
                        "Content-Type": "application/json",
                        "Access-Control-Allow-Origin": "*"
                    },
                    withCredentials: true,
                }
            );
            console.log(response?.data?.token);

            this.setState ({
                username: "",
                password: ""
            });
        } catch (err) {
            console.log("Login Failed")
        }
    };

    navigateToHome = e => {
        window.location("http://localhost:3000/")
    }

    render() {
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
                <form onSubmit={this.navigteToHome}>
                <div className="input">
                    <input type="text"
                        placeholder="Username"
                        //value={this.state.username}
                        onChange={this.handleOnChange}/>
                </div>
                <div className="input">
                    <input type="password"
                        placeholder="Password"
                        //value={this.state.password}
                        onChange={this.handleOnChange}/>
                </div>
                <div className="submit-container">
                    <button type="submit">LOG IN</button>
                </div>
                </form>
            </div>
            </div>
            </div>
        );
    };
}

export default LoginSignup;
