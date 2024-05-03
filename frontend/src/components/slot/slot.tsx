import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemText from '@mui/material/ListItemText';
import ListItemAvatar from '@mui/material/ListItemAvatar';
import Avatar from '@mui/material/Avatar';
import DirectionsCarIcon from '@mui/icons-material/DirectionsCar';

export default function Slot({ spotId, arriveTime }: { spotId: number, arriveTime: string }) {
  const date = new Date(arriveTime).toLocaleString()
  return (
    <List sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper' }}>
      <ListItem>
        <ListItemAvatar>
          <Avatar>
            <DirectionsCarIcon />
          </Avatar>
        </ListItemAvatar>
        <ListItemText primary={`Slot ${spotId} taken`} secondary={`Occupied from ${date}`} />
      </ListItem>
    </List>
  );
}
