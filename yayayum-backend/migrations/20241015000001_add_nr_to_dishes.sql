-- Add nr field to existing dishes table
-- Migration: 20241015000001_add_nr_to_dishes

-- Only add the column if it doesn't exist
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name='dishes' AND column_name='nr') THEN
        ALTER TABLE dishes ADD COLUMN nr INTEGER;
        
        -- Update existing dishes to have a default nr value
        UPDATE dishes SET nr = id WHERE nr IS NULL;
        
        -- Make the nr column NOT NULL after setting values
        ALTER TABLE dishes ALTER COLUMN nr SET NOT NULL;
    END IF;
END $$;