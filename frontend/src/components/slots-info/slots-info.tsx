import List from '@mui/material/List';
import Slot from '../slot/slot';
import { useQuery } from '@tanstack/react-query';
import { ICar } from '../../types/car.dto';

const fetchCarData = async (): Promise<ICar[]> => {
  const data = fetch('http://localhost:3000/cars?current=true').then((res) =>
    res.json(),
  )
  return data
}


export default function SlotsInfoList() {

  const { status, data: carData, error } = useQuery({
    queryKey: ['cars'],
    queryFn: fetchCarData,
  })


  if (status === 'pending') {
    return <span>Loading...</span>
  }

  if (status === 'error') {
    return <span>Error: {error.message}</span>
  }

  return (
    <List sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper' }}>
      {
        carData.map(car => <Slot arriveTime={car.car_arrived} spotId={car.spot_id} key={car.id} />)
      }
    </List>
  );
}
