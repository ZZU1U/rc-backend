-- Add migration script here
ALTER TABLE car
ADD power INT;
COMMENT ON COLUMN car.power IS 'car max power for speed';
