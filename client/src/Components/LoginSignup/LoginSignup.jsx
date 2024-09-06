import './LoginSignup.css'

import google_logo from '../Assets/google_logo.png'
import apple_logo from '../Assets/apple_logo.png'
import login_background from '../Assets/login_background.jpg'

const LoginSignup = () => {
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
        <div class="line-container">
            <p><span>OR</span></p>
        </div> 
        <div className="inputs">
            <div className="input">
                <input type="text" placeholder="Username" />
            </div>
            <div className="input">
                <input type="password" placeholder="Password" />
            </div>
        </div>
        <div className="submit-container">
            <button>LOG IN</button>
        </div>
        </div>
        </div>
    );
}

export default LoginSignup;