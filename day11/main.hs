import Data.Array

data Cell = Floor | Empty | Occupied
  deriving (Show, Eq)

cellFromString 'L' = Empty
cellFromString '#' = Occupied
cellFromString '.' = Floor

stringFromCell Empty = 'L'
stringFromCell Occupied = '#'
stringFromCell Floor = '.'

directions = [(-1, -1), (-1, 0), (-1, 1),
              (0,  -1),          (0, 1),
              (1, -1),  (1, 0),  (1, 1)]
offsetBy col row distance (colOff, rowOff) = (col + colOff * distance, row + rowOff * distance)
neighboursAt col row = offsetBy col row 1 <$> directions
inBounds width height (col, row) = col >= 0 && col < width && row >= 0 && row < height

findRoom n (room1:room2:rooms) 
  | room1 == room2 = do putStrLn $ "Done " ++ show n
                        pure room1
  | otherwise = do putStrLn $ "Working.... " ++ show n
                   --putStrLn $ concat $ fmap (('\n':) . fmap stringFromCell . elems) $ elems room1
                   findRoom (n+1) (room2:rooms)

neighbours :: Int -> Int -> Int -> Int -> Array Int (Array Int Cell) -> [Cell]
neighbours width height col row waitingRoom = (\(col', row') -> waitingRoom ! row' ! col') <$> filter (inBounds width height) (neighboursAt col row)

visible :: Int -> Int -> Int -> Int -> Array Int (Array Int Cell) -> [Cell]
visible width height col row waitingRoom = concat (firstVisible . inDirection <$> directions)
  where
    inDirection :: (Int, Int) -> [(Int, Int)]
    inDirection offset = takeWhile (inBounds width height)
                                   ((\d -> offsetBy col row d offset) <$> [1..])
    firstVisible :: [(Int, Int)] -> [Cell]
    firstVisible = take 1 . filter (/= Floor) . fmap (\(col', row') -> waitingRoom ! row' ! col')

next :: Int -> (Int -> Int -> Int -> Int -> Array Int (Array Int Cell) -> [Cell]) -> Int -> Int -> Array Int (Array Int Cell) -> Array Int (Array Int Cell)
next threshold f width height waitingRoom
  = listArray (0, height - 1) 
              [ listArray (0, width - 1)
                          [ let influential = f width height col row waitingRoom
                             in case waitingRoom ! row ! col of
                                  Empty -> if any (== Occupied) influential then Empty else Occupied
                                  Occupied -> if (length . filter (== Occupied) $ influential) >= threshold then Empty else Occupied
                                  Floor -> Floor
                          | col <- indices $ waitingRoom ! row
                          ]
              | row <- indices waitingRoom
              ]

main = do
  inputLines <- lines <$> getContents
  let width = fromIntegral $ length (head inputLines)
  let height = fromIntegral $ length inputLines
  let waitingRoom = listArray (0, height - 1) [ listArray (0, width - 1) [ cellFromString inputChar 
                                                                         | inputChar <- inputLine]
                                              | inputLine <- inputLines]
  let rooms = iterate (next 4 neighbours width height) waitingRoom
  room <- findRoom 0 rooms
  putStrLn $ show $ length $ filter (== Occupied) $ concat $ fmap elems $ elems room

  let rooms = iterate (next 5 visible width height) waitingRoom
  room <- findRoom 0 rooms
  putStrLn $ show $ length $ filter (== Occupied) $ concat $ fmap elems $ elems room
