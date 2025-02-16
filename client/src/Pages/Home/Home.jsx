import './Home.css';
import Sidebar from '../../Components/Sidebar/Sidebar'

const Home = () => {
	return (
		<div className='main__div'>
			<div className='left__sidebar'>
				<Sidebar/>
			</div>
			<div className='right__body'>
			</div>
		</div>
	);
};

export default Home;
