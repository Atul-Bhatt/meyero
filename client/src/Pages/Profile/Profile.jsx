import './Profile.css';
import Navbar from '../../Components/Navbar/Navbar';
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { APP_ROUTES } from '../../utils/constants';
import { Dialog, DialogTitle, DialogContent, Grid, Avatar, IconButton } from '@mui/material';

const avatarList = [
  'one.svg',
  'two.svg',
  'three.svg',
  'four.svg',
  'five.svg',
  'six.svg',
  'seven.svg',
  'eight.svg',
  'nine.svg',
  'ten.svg',
]; // add filenames from /public/avatars

const Profile = () => {
  const navigate = useNavigate();
  const [open, setOpen] = useState(false);
  const [selectedAvatar, setSelectedAvatar] = useState(null);

  useEffect(() => {
    if (!localStorage.getItem('token')) {
      navigate(APP_ROUTES.LOG_IN);
    }
  }, [navigate]);

  const handleAvatarSelect = (avatar) => {
    setSelectedAvatar(avatar);
    setOpen(false);
    // You can also save it to backend or localStorage here
  };

  return (
    <div className="container">
      <div className="navbar">
        <Navbar />
      </div>
      <div className="container__body">
        <div className="main__body">
          <p>Change Profile Avatar</p>
          <Avatar
            src={selectedAvatar ? `/avatars/${selectedAvatar}` : ''}
            sx={{ width: 80, height: 80, cursor: 'pointer' }}
            onClick={() => setOpen(true)}
          />

          {/* Avatar Selection Dialog */}
          <Dialog open={open} onClose={() => setOpen(false)}>
            <DialogTitle>Select an Avatar</DialogTitle>
            <DialogContent>
              <Grid container spacing={2}>
                {avatarList.map((avatar) => (
                  <Grid item key={avatar}>
                    <IconButton onClick={() => handleAvatarSelect(avatar)}>
                      <Avatar
                        src={`/avatars/${avatar}`}
                        sx={{ width: 60, height: 60 }}
                      />
                    </IconButton>
                  </Grid>
                ))}
              </Grid>
            </DialogContent>
          </Dialog>
        </div>
      </div>
    </div>
  );
};

export default Profile;
