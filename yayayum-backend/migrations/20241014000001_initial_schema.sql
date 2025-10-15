-- Initial schema for YayaYum database-- Initial schema for YayaYum database

-- Migration: 20241014000001_initial_schema-- Migration: 20241014000001_initial_schema



-- Create users table-- Create users table

CREATE TABLE IF NOT EXISTS users (CREATE TABLE IF NOT EXISTS users (

    id SERIAL PRIMARY KEY,    id SERIAL PRIMARY KEY,

    username TEXT NOT NULL    username TEXT NOT NULL

););



-- Create dishes table  -- Create dishes table  

CREATE TABLE IF NOT EXISTS dishes (CREATE TABLE IF NOT EXISTS dishes (

    id SERIAL PRIMARY KEY,    id SERIAL PRIMARY KEY,

    name TEXT NOT NULL,    name TEXT NOT NULL,

    description TEXT NOT NULL,    description TEXT NOT NULL,

    price_kr INTEGER NOT NULL,    price_kr INTEGER NOT NULL,

    dietary_restrictions TEXT NOT NULL,    dietary_restrictions TEXT NOT NULL,

    category TEXT NOT NULL    category TEXT NOT NULL

););



-- Create indexes for performance-- Create indexes for performance

CREATE INDEX IF NOT EXISTS idx_dishes_category ON dishes(category);CREATE INDEX IF NOT EXISTS idx_dishes_category ON dishes(category);

CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);