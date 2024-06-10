const convert100to4095 = (value: number): number => {
  return value * 4095 / 100;
}

export function calculateWheelDirections(x: number, y: number, maxInput: number = 100): number[] {
  // Input validation
  if (Math.abs(x) > maxInput || Math.abs(y) > maxInput) {
    throw new Error("Joystick/controller input values must be between -maxInput and maxInput");
  }

  // Scaling factors
  const maxSpeed = 4095;  // Maximum speed value for the wheels
  const minSpeed = -4095;  // Minimum speed to apply to the inner wheels to prevent them from being 0
  const minSpeedDuringTurn = 500;  // Minimum speed to apply to the inner wheels during a turn
  
  let frontLeft = 0;
  let backLeft = 0;
  let frontRight = 0;
  let backRight = 0;

  // X is forward and backward, Y is left and right
  // to calculate the speed of the inner wheels, we use the
  // formula: speed = (y - x) * maxSpeed / maxInput

  if (x !== 0 && y === 0) {
    // Move forward or backward
    frontLeft = convert100to4095(x);
    backLeft = convert100to4095(x);
    frontRight = convert100to4095(x);
    backRight = convert100to4095(x);
  }
  if (x === 0 && y !== 0) {
    // means it's a donut turn
    // if y is positive, donut right
    // if y is negative, donut left
    if (y > 0) {
      frontLeft = maxSpeed;
      backLeft = maxSpeed;
      frontRight = -maxSpeed;
      backRight = -maxSpeed;
    } else {
      frontLeft = -maxSpeed;
      backLeft = -maxSpeed;
      frontRight = maxSpeed;
      backRight = maxSpeed;
    }
  }

  if (x !== 0 && y !== 0) {
    // Means we're turning
    // if y is positive, turn right
    // if y is negative, turn left
    // we calculate the speed of the inner wheels using the x as a maxSpeed and y as the intensity of the turn
    // we also apply a minimum speed to the inner wheels to prevent them from being 0

    // Turning movement
    const xSpeed = (x / maxInput) * maxSpeed;
    const turnIntensity = Math.abs(y) / maxInput; // Calculate intensity of the turn
    const adjustedTurnIntensity = turnIntensity > 0.9 ? 0.9 : turnIntensity; // Cap the turn intensity to prevent 0 speed

    if (y > 0) {
      // Turning right
      frontLeft = Math.round(xSpeed);
      backLeft = Math.round(xSpeed);
      frontRight = Math.round(xSpeed * (1 - adjustedTurnIntensity) + minSpeed);
      backRight = Math.round(xSpeed * (1 - adjustedTurnIntensity) + minSpeed);
    } else {
      // Turning left
      frontLeft = Math.round(xSpeed * (1 - adjustedTurnIntensity) + minSpeed);
      backLeft = Math.round(xSpeed * (1 - adjustedTurnIntensity) + minSpeed);
      frontRight = Math.round(xSpeed);
      backRight = Math.round(xSpeed);
    }
  }

    return [frontLeft, backLeft, frontRight, backRight];
  
}
  

