import Timeline from '@mui/lab/Timeline';
import TimelineItem from '@mui/lab/TimelineItem';
import TimelineSeparator from '@mui/lab/TimelineSeparator';
import TimelineConnector from '@mui/lab/TimelineConnector';
import TimelineContent from '@mui/lab/TimelineContent';
import TimelineDot from '@mui/lab/TimelineDot';
import TimelineOppositeContent from '@mui/lab/TimelineOppositeContent';
import { useQuery } from '@tanstack/react-query';
import { ICar } from '../../types/car.dto';

const fetchCarData = async (): Promise<ICar[]> => {
  const data = fetch(`http://${import.meta.env.VITE_BackendAddress}/cars`).then((res) =>
    res.json(),
  )
  return data
}

const sortCarsByLastAction = (cars: ICar[]) => {
  cars.sort((a, b) => {
    const dateA = a.car_left ? new Date(a.car_left) : new Date(a.car_arrived)
    const dateB = b.car_left ? new Date(b.car_left) : new Date(b.car_arrived)
    return dateB.getTime() - dateA.getTime();
  });

  return cars
}

export default function History() {
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
    < Timeline position="alternate" >
      {sortCarsByLastAction(carData).map((car, index) => (
        <TimelineItem key={car.id}>
          <TimelineOppositeContent color="text.secondary">
            {car.car_left ? new Date(car.car_left).toLocaleString() : new Date(car.car_arrived).toLocaleString()}
          </TimelineOppositeContent>
          <TimelineSeparator>
            <TimelineDot />
            {index !== carData.length - 1 ? <TimelineConnector /> : <></>}
          </TimelineSeparator>
          <TimelineContent>Car {car.car_left ? 'Left' : 'Arrived'} at spot {car.spot_id}</TimelineContent>
        </TimelineItem>
      ))}
    </Timeline >
  );
}
