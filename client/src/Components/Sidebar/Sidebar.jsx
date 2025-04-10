import './Sidebar.css';
import { API_ROUTES } from '../../utils/constants'

import React, { useState, useEffect } from 'react';
import People from '../People/People';
import axios from 'axios';

const Sidebar = ({set_user_id}) => {
    const [users, setUsers] = useState([]);

    useEffect(() => {
		const token = localStorage.getItem("token");
		if (token) {
			axios.defaults.headers.common["Authorization"] = `Bearer ${token}`
		}
        axios.get(API_ROUTES.GET_USER_LIST)
            .then(response => {
                setUsers(response.data.user)
            })
            .catch(error => {
                console.log(error)
            });
    }, []);

    return (
        <div className='sidebar__list'>
            {/* List of users */}
            <div className='people__list'>
                {users.map(user => (
                    <People name={user.name} id={user.id} set_user_id={set_user_id}/>
                ))}
            </div>
        </div>   
    );
}

export default Sidebar;
