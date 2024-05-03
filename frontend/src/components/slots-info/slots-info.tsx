import List from '@mui/material/List';
import Slot from '../slot/slot';
import { useQuery } from '@tanstack/react-query';
import { ICar } from '../../types/car.dto';

const fetchCurrentCarData = async (): Promise<ICar[]> => {
  const data = fetch('http://localhost:3000/cars?current=true').then((res) =>
    res.json(),
  )
  return data
}

const sortCarsBySpotId = (cars: ICar[]) => (
  cars.sort((a, b) => a.spot_id - b.spot_id)
)

export default function SlotsInfoList() {

  const { status, data: carData, error } = useQuery({
    queryKey: ['current-cars'],
    queryFn: fetchCurrentCarData,
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
        sortCarsBySpotId(carData).map(car => <Slot arriveTime={car.car_arrived} spotId={car.spot_id} key={car.id} />)
      }
    </List>
  );
}
