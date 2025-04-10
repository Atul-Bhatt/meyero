import './Home.css';
import Sidebar from '../../Components/Sidebar/Sidebar'
import WebSocket from '../../WebSocket/WebSocket';
import { useState } from 'react';

const Home = () => {
	const [userId, setUserId] = useState("")	
	return (
		<div className='container'>
			<div className='navbar'></div>
			<div className='container__body'>
				<div className='sidebar'>
					<Sidebar set_user_id={setUserId}/>
				</div>
				<div className='main__body'>
					<WebSocket user_id={userId}/>
				</div>
			</div>
		</div>
	);
};

export default Home;
