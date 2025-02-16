import './Home.css';
import Sidebar from '../../Components/Sidebar/Sidebar'

const Home = () => {
	return (
		<div className='container'>
			<div className='navbar'></div>
			<div className='body'>
				<div className='sidebar'>
					<Sidebar/>
				</div>
				<div className='main__body'>
				</div>
			</div>
		</div>
	);
};

export default Home;
