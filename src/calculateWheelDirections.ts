export function calculateWheelDirections(x: number, y: number, maxInput: number = 100): number[] {
  // Input validation
  if (Math.abs(x) > maxInput || Math.abs(y) > maxInput) {
    throw new Error("Joystick/controller input values must be between -maxInput and maxInput");
  }

  // Scaling factors
  const maxSpeed = 4095;  // Maximum speed value for the wheels
  const minSpeed = 300;  // Minimum speed to apply to the inner wheels to prevent them from being 0

  let frontLeft = 0;
  let backLeft = 0;
  let frontRight = 0;
  let backRight = 0;

  if (x !== 0 && y === 0) {
    // Straight movement
    const speed = Math.round((x / maxInput) * maxSpeed);
    frontLeft = speed;
    backLeft = speed;
    frontRight = speed;
    backRight = speed;
  } else if (x !== 0 && y !== 0) {
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

  // Clamping the speeds to the allowed range [-4095, 4095]
  frontLeft = Math.max(-maxSpeed, Math.min(maxSpeed, frontLeft));
  backLeft = Math.max(-maxSpeed, Math.min(maxSpeed, backLeft));
  frontRight = Math.max(-maxSpeed, Math.min(maxSpeed, frontRight));
  backRight = Math.max(-maxSpeed, Math.min(maxSpeed, backRight));

  return [frontLeft, frontRight, backRight, backLeft];
}