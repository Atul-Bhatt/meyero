import './Navbar.css';
import Avatar from '@mui/material/Avatar'

const Navbar = ({photoUrl, userName}) => {
	return (
    <nav className="navbar">
      <a href="/" className="navbar-logo">Meyero</a>

      <input type="text" className="navbar-search" placeholder="Search users..." />

      <div className="navbar-user">
        <Avatar src={photoUrl} />
        <div className="navbar-name">John Doe</div>
        <div className="navbar-dropdown">
          <a href="/profile">Profile</a>
          <a href="/settings">Settings</a>
          <a href="/logout">Logout</a>
        </div>
      </div>
    </nav>
	);
};

export default Navbar;