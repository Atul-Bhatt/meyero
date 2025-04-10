import './Home.css';
import Sidebar from '../../Components/Sidebar/Sidebar'
import WebSocket from '../../WebSocket/WebSocket';
import { useState } from 'react';

const Home = () => {
	const [currentUser, setCurrentUser] = useState("")	
	return (
		<div className='container'>
			<div className='navbar'></div>
			<div className='container__body'>
				<div className='sidebar'>
					<Sidebar setCurrentUser={setCurrentUser}/>
				</div>
				<div className='main__body'>
					<WebSocket currentUser={currentUser}/>
				</div>
			</div>
		</div>
	);
};

export default Home;
