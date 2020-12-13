{-# LANGUAGE TemplateHaskell #-}
 
import Prelude hiding (Either (..))
import Control.Lens
import Debug.Trace

newtype Distance = Distance Int
  deriving (Show)

newtype Angle = Angle Int
  deriving (Show)

data Direction = North | East | South | West
  deriving (Show)

data Side = Left | Right
  deriving (Show)

data Command = MoveDirection Direction Distance
             | MoveForward Distance
             | Turn Side Angle
  deriving (Show)

data State = State { _stateDirection :: Direction, _stateX :: Int, _stateY :: Int, _stateWayX :: Int, _stateWayY :: Int }
  deriving (Show)

makeLenses ''State

startState = State { _stateDirection = East, _stateX = 0, _stateY = 0, _stateWayX = 10, _stateWayY = 1 }

turnOnce Left North = West
turnOnce Left West = South
turnOnce Left South = East
turnOnce Left East = North
turnOnce Right North = East
turnOnce Right East = South
turnOnce Right South = West
turnOnce Right West = North

turnWaypointOnce d state = let (newX, newY) = go (state ^. stateWayX) (state ^. stateWayY)
                            in set stateWayX newX . set stateWayY newY $ state
  where go wayX wayY | wayX >= 0 && wayY >= 0 = case d of -- NE quadrant
                                                  Left -> (-wayY, wayX)
                                                  Right -> (wayY, -wayX)
                     | wayX >= 0 && wayY < 0  = case d of -- SE quadrant
                                                  Left -> (-wayY, wayX)
                                                  Right -> (wayY, -wayX)
                     | wayX < 0 && wayY < 0   = case d of -- SW quadrant
                                                  Left -> (-wayY, wayX)
                                                  Right -> (wayY, -wayX)
                     | wayX < 0 && wayY >= 0  = case d of -- NW quadrant
                                                  Left -> (-wayY, wayX)
                                                  Right -> (wayY, -wayX)

applyCommand :: Command -> State -> State
applyCommand (MoveDirection East (Distance distance)) = over stateX (+ distance)
applyCommand (MoveDirection West (Distance distance)) = over stateX (+ (-distance))
applyCommand (MoveDirection North (Distance distance)) = over stateY (+ distance)
applyCommand (MoveDirection South (Distance distance)) = over stateY (+ (-distance))
applyCommand (MoveForward distance) = \state -> applyCommand (MoveDirection (state ^. stateDirection) distance) state
applyCommand (Turn _ (Angle 0)) = id
applyCommand (Turn side (Angle angle)) = applyCommand (Turn side (Angle $ angle - 90)) . over stateDirection (turnOnce side)


applyCommandWaypoint :: Command -> State -> State
applyCommandWaypoint (MoveDirection East (Distance distance)) = over stateWayX (+ distance)
applyCommandWaypoint (MoveDirection West (Distance distance)) = over stateWayX (+ (-distance))
applyCommandWaypoint (MoveDirection North (Distance distance)) = over stateWayY (+ distance)
applyCommandWaypoint (MoveDirection South (Distance distance)) = over stateWayY (+ (-distance))
applyCommandWaypoint (MoveForward (Distance distance)) = \state -> over stateX (+ (state ^. stateWayX) * distance) . over stateY (+ (state ^. stateWayY) * distance) $ state
applyCommandWaypoint (Turn _ (Angle 0)) = id
applyCommandWaypoint (Turn side (Angle angle)) = applyCommandWaypoint (Turn side (Angle $ angle - 90)) . turnWaypointOnce side



readCommand commandString = let commandChar = head commandString
                                commandVal = read (tail commandString)
                             in case commandChar of
                                  'N' -> MoveDirection North (Distance commandVal)
                                  'E' -> MoveDirection East  (Distance commandVal)
                                  'S' -> MoveDirection South (Distance commandVal)
                                  'W' -> MoveDirection West  (Distance commandVal)
                                  'L' -> Turn Left (Angle commandVal)
                                  'R' -> Turn Right (Angle commandVal)
                                  'F' -> MoveForward (Distance commandVal)

main = do
  commands <- (fmap readCommand . lines) <$> getContents
  let endState = foldl (flip applyCommand) startState commands
  putStrLn . show $ (abs $ endState ^. stateX) + (abs $ endState ^. stateY)

  let endState = foldl (flip applyCommandWaypoint) startState commands
  putStrLn . show $ (abs $ endState ^. stateX) + (abs $ endState ^. stateY)

