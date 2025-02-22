import './People.css'
import Avatar from '@mui/material/Avatar'

const People = ({ name, photoUrl }) => {
  return (
    <div className='people__main'>
      <Avatar src={photoUrl} />
      <h3>{name}</h3>
    </div>
  )
}

export default People
