import './Home.css';
import Sidebar from '../../Components/Sidebar/Sidebar'
import Chat from '../../Components/Chat/Chat'

const Home = () => {
	return (
		<div className='container'>
			<div className='navbar'></div>
			<div className='container__body'>
				<div className='sidebar'>
					<Sidebar/>
				</div>
				<div className='main__body'>
					<Chat/>
				</div>
			</div>
		</div>
	);
};

export default Home;
