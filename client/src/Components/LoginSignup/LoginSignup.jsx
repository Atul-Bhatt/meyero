import './LoginSignup.css'

import google_logo from '../Assets/google_logo.png'
import apple_logo from '../Assets/apple_logo.png'

const LoginSignup = () => {
    return (
        <div className='container'>
        <div className="header">
            <div className="text">Log in</div>
        </div>
        <div className="otherlogins">
            <button className="otherLoginButton">
                <img src={google_logo} alt="google_logo"></img>
                Continue with Google
            </button>
            <br/>
            <button className="otherLoginButton">
                <img src={apple_logo} alt="apple_logo" />
                Continue with Apple
            </button>
        </div>
        <p>OR</p>
        <div className="inputs">
            <div className="input">
                <input type="text" />
            </div>
            <div className="input">
                <input type="password" />
            </div>
        </div>
        <button className="loginButton">
            LOG IN
        </button>
        <div className="forgotUsernamePassword">
           Forgot your <a>username</a> or <a>password</a>?
        </div>
        <div className="goToSignUp">
            New to Meyero? <a>SIGN UP</a>
        </div>
        </div>
    );
}

export default LoginSignup;