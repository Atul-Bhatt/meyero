import './Home.css';
import Sidebar from '../../Components/Sidebar/Sidebar'
import WebSocket from '../../WebSocket/WebSocket';
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom'
import { APP_ROUTES } from '../../utils/constants'

const Home = () => {
	const [otherUser, setOtherUser] = useState("")	
	const [canvas, setCanvas] = useState("")
	const navigate = useNavigate()

	useEffect(()=>{
        if (localStorage.getItem("token") == undefined) {
            navigate(APP_ROUTES.LOG_IN)
        }
	},[])

	return (
		<div className='container'>
			<div className='navbar'></div>
			<div className='container__body'>
				<div className='sidebar'>
					<Sidebar setOtherUser={setOtherUser} setCanvas={setCanvas}/>
				</div>
				<div className='main__body'>
					<WebSocket otherUser={otherUser} canvas={canvas}/>
				</div>
			</div>
		</div>
	);
};

export default Home;
