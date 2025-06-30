import './Navbar.css';
import Avatar from '@mui/material/Avatar'
import { useState, useEffect, useRef } from 'react';
import { API_ROUTES } from '../../utils/constants'
import axios from 'axios';
import People from '../People/People';

const Navbar = ({photoUrl, userName}) => {
    const handleLogOut = () => {
        localStorage.removeItem("token")
        localStorage.removeItem("userId")
        localStorage.removeItem("userName")
    }

    const [searchText, setSearchText] = useState('');
	const [users, setUsers] = useState([]);
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
    <nav className="navbar">
      <a href="/" className="navbar-logo">Meyero</a>

      <input ref={dropdownRef}
		type="text"
		className="navbar-search"
		placeholder="Search users..."
		onChange={(event) => setSearchText(event.target.value)}
		onKeyDown={(event) => { if (event.key === 'Enter') { handleSearch(event); }}}/>

	  {users.length > 0 && (
		<div className='search-dropdown'>
			{users.map(user => (
				<People user={user}/>
			))}
		</div>
	  )}

      <div className="navbar-user">
        <Avatar src={photoUrl} />
        <div className="navbar-name">{localStorage.getItem("userName")}</div>
        <div className="navbar-dropdown">
          <a href="/profile">Profile</a>
          <a href="/login" onClick={handleLogOut}>Logout</a>
        </div>
      </div>
    </nav>
	);
};

export default Navbar;
