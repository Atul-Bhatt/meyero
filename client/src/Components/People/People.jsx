import './People.css'
import Avatar from '@mui/material/Avatar'
import { API_ROUTES } from '../../utils/constants'
import axios from 'axios';
import {v4 as uuidv4} from 'uuid';
import google_logo from '../Assets/google_logo.png';

const People = ({ user, photoUrl, setOtherUser, setCanvas}) => {
	const openChat = async (event) => {
		const token = localStorage.getItem("token");
		if (token) {
			axios.defaults.headers.common["Authorization"] = `Bearer ${token}`
		}
    event.preventDefault();

    try {
      const now = new Date();
      const response = await axios.post(API_ROUTES.INITIATE_CHAT, {
        "id": uuidv4(),
        "from_user": localStorage.getItem("userId"),
        "to_user": user.id,
        "created_at": now.toISOString(),
        "updated_at": now.toISOString()
    });
    console.log(response)
    let canvas = {
      received: response.data.receive_canvas,
      sent: response.data.send_canvas
    }
    setCanvas(canvas)
    } catch(error) {
        console.log(error);
    }

    setOtherUser(user)
  };

  return (
    <div className='people__main' onClick={openChat}>
      <Avatar src={`./avatars/${user.avatar}.svg`}  />
      <h3>{user.name}</h3>
    </div>
  )
}

export default People
