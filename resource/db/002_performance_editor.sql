USE personal_website;

-- 1. IMAGE Table
CREATE TABLE IF NOT EXISTS image (
    id VARCHAR(36) PRIMARY KEY,
    profile_id VARCHAR(36) NOT NULL,
    
    -- File info
    filename VARCHAR(255) NOT NULL,              -- UUID-based: "abc123.jpg"
    original_filename VARCHAR(255) NOT NULL,     -- User's original: "vacation.jpg"
    storage_url TEXT NOT NULL,                   -- GCS URL
    
    -- Image properties
    file_size INT NOT NULL,                      -- bytes
    width INT,                                   -- pixels
    height INT,                                  -- pixels
    mime_type VARCHAR(50) NOT NULL,              -- "image/jpeg", "image/png", etc.
    
    -- Optional metadata
    alt_text VARCHAR(255),                       -- For accessibility
    caption TEXT,                                -- Description
    
    created_at DATETIME NOT NULL,
    
    FOREIGN KEY (profile_id) REFERENCES profile(id) ON DELETE CASCADE,
    INDEX idx_profile_created (profile_id, created_at DESC)
);

-- 2. IMAGE_USAGE Table
CREATE TABLE IF NOT EXISTS image_usage (
    id VARCHAR(36) PRIMARY KEY,
    image_id VARCHAR(36) NOT NULL,
    performance_id VARCHAR(36) NOT NULL,
    
    usage_count INT DEFAULT 1,                   -- How many times in this performance
    first_used_at DATETIME NOT NULL,
    last_used_at DATETIME NOT NULL,
    
    FOREIGN KEY (image_id) REFERENCES image(id) ON DELETE CASCADE,
    FOREIGN KEY (performance_id) REFERENCES performance(id) ON DELETE CASCADE,
    
    UNIQUE KEY unique_image_performance (image_id, performance_id),
    INDEX idx_image_usage (image_id),
    INDEX idx_performance_images (performance_id)
);

-- 3. IMAGE_TAG Table (Optional - but included for completeness as per design)
CREATE TABLE IF NOT EXISTS image_tag (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,            -- "screenshot", "diagram", etc.
    color VARCHAR(20)                            -- For UI display
);

CREATE TABLE IF NOT EXISTS image_tag_list (
    image_id VARCHAR(36) NOT NULL,
    tag_id VARCHAR(36) NOT NULL,
    
    PRIMARY KEY (image_id, tag_id),
    FOREIGN KEY (image_id) REFERENCES image(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES image_tag(id) ON DELETE CASCADE
);
