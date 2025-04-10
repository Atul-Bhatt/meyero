import './Home.css';
import Sidebar from '../../Components/Sidebar/Sidebar'
import WebSocket from '../../WebSocket/WebSocket';
import { useState } from 'react';

const Home = () => {
	const [currentUser, setCurrentUser] = useState("")	
	const [canvas, setCanvas] = useState("")
	return (
		<div className='container'>
			<div className='navbar'></div>
			<div className='container__body'>
				<div className='sidebar'>
					<Sidebar setCurrentUser={setCurrentUser} setCanvas={setCanvas}/>
				</div>
				<div className='main__body'>
					<WebSocket currentUser={currentUser} canvas={canvas}/>
				</div>
			</div>
		</div>
	);
};

export default Home;
