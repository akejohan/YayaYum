-- Initial schema for YayaYum database
-- Migration: 20241014000001_initial_schema

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL
);

-- Create dishes table  
CREATE TABLE IF NOT EXISTS dishes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    price_kr INTEGER NOT NULL,
    dietary_restrictions TEXT NOT NULL,
    category TEXT NOT NULL
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_dishes_category ON dishes(category);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);