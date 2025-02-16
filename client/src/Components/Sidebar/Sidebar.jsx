import React, { useState, useEffect } from 'react';
import './Sidebar.css';
import People from '../People/People';
import axios from 'axios';

const Sidebar = () => {
    const [users, setUsers] = useState([]);
    const url = "http://localhost:8081/user/list";

    useEffect(() => {
        axios.get(url)
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
                    <People name={user.name}/>
                ))}
            </div>
        </div>   
    );
}

export default Sidebar;