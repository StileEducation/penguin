require_relative "../penguin_object_id"

class Penguin::ObjectId
  include Comparable

  def self.new
    Penguin::ObjectId.generate
  end

  def self.from_time(time, unique: true)
    Penguin::ObjectId.generate_from_time(time, unique)
  end
end