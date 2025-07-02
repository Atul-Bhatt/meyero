import './Group.css';
import { useState, useEffect, useRef } from 'react';
import { API_ROUTES } from '../../utils/constants'
import axios from 'axios';
import People from '../../Components/People/People';

const Group = () => {
    const [searchText, setSearchText] = useState('');
	const [users, setUsers] = useState([]);
    const [addedUsers, setAddedUsers] = useState([]);

	const handleSearch = (event) => {
        event.preventDefault();

		const token = localStorage.getItem("token");
		if (token) {
			axios.defaults.headers.common["Authorization"] = `Bearer ${token}`
		}

		axios.get(API_ROUTES.SEARCH, {
			params: { username: searchText }
		})
		.then(response => {
			setUsers(response.data.data.users)
		})
		.catch(error => {
			console.log(error)
		});
	};

	const dropdownRef = useRef();

	useEffect(() => {
	  function handleClickOutside(event) {
		if (dropdownRef.current && !dropdownRef.current.contains(event.target)) {
			setUsers([]);
			dropdownRef.current.value = '';
		}
	  }

	  document.addEventListener("mousedown", handleClickOutside);
	  return () => {
		document.removeEventListener("mousedown", handleClickOutside);
	  };
	}, []);

	return (
        <div className="group-container">
            <div>
            <h3>Create a new group</h3>
            <div>
                <form>
                <div>
                <input type="text" placeholder="group name"></input>
                </div>
                <div>
                    <button type="submit">Create Group</button>
                </div>
                </form>
            </div>
            <div className='added-user-list'>
                {addedUsers.map(user => (
                    <People user={user}/>
                ))}
            </div>
            </div>
            <div ref={dropdownRef} className="user-search">
            <input 
                type="text"
                className="user-search-input"
                placeholder="Search users..."
                onChange={(event) => setSearchText(event.target.value)}
                onKeyDown={(event) => { if (event.key === 'Enter') { handleSearch(event); }}}/>

            {users.length > 0 && (
                <div className='search-dropdown'>
                    {users.map(user => (
                        <People user={user} onClick={() => setAddedUsers(prev => [...prev, user])}/>
                    ))}
                </div>
            )}
            </div>
        </div>
	);
};

export default Group;