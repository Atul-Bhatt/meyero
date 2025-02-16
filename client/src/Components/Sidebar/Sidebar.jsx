import './Sidebar.css';
import People from '../People/People';
import axios from 'axios';

const Sidebar = () => {

    const url = "http://localhost:8081/user/list";
    function fetchUsers() {
        try {
            const response = axios.get(url);
            const users = response.data;
            console.log(users);
        } catch(error) {
            console.log(error)
        }
    }
    fetchUsers();

    return (
        <div className='sidebar__list'>
            {/* List of users */}
            <div className='people__list'>
                <People
                    key={'id'}
                    name={'name'}
                />
            </div>
        </div>   
    );
}

export default Sidebar;